//! The sampler: one thread, one `sysinfo::System`, one channel to the interface.

pub mod group;

use crate::model::{
    Capabilities, CpuStat, DiskIo, DiskStat, GpuStat, MemStat, NetStat, Priority, ProcKey, ProcRow,
    SessionRow, Snapshot, Status,
};
use crate::platform::{self, RealRunner, Runner};
use crate::store::history::History;
use group::Classifier;
use std::collections::HashMap;

/// A counter may reset (device replaced, adapter recreated). A decrease means
/// zero for this sample, never a negative or absurd rate.
pub fn delta_rate(previous: u64, current: u64, dt_secs: f64) -> f64 {
    if dt_secs <= 0.0 || current < previous {
        0.0
    } else {
        (current - previous) as f64 / dt_secs
    }
}
use std::sync::mpsc::{self, Receiver, RecvTimeoutError, Sender};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};
use sysinfo::{
    Disks, Networks, Pid, ProcessRefreshKind, ProcessStatus, ProcessesToUpdate, System, UpdateKind,
    Users,
};

const SERVICES_TTL: Duration = Duration::from_secs(5);
const HISTORY_FLUSH_EVERY: Duration = Duration::from_secs(30);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    SetInterval(Option<Duration>),
    RefreshNow,
    ClearHistory,
    Stop,
}

pub struct Sampler {
    pub snapshots: Receiver<Snapshot>,
    commands: Sender<Command>,
    handle: Option<JoinHandle<()>>,
}

impl Sampler {
    pub fn spawn(interval: Option<Duration>) -> Result<Self, String> {
        let inner = Inner::new()?;
        let (snapshot_tx, snapshot_rx) = mpsc::channel();
        let (command_tx, command_rx) = mpsc::channel();
        let handle = std::thread::Builder::new()
            .name("sampler".to_string())
            .spawn(move || run(inner, interval, snapshot_tx, command_rx))
            .map_err(|err| format!("cannot start the sampler thread: {err}"))?;
        Ok(Self {
            snapshots: snapshot_rx,
            commands: command_tx,
            handle: Some(handle),
        })
    }

    pub fn set_interval(&self, interval: Option<Duration>) {
        let _ = self.commands.send(Command::SetInterval(interval));
    }

    pub fn refresh_now(&self) {
        let _ = self.commands.send(Command::RefreshNow);
    }

    pub fn clear_history(&self) {
        let _ = self.commands.send(Command::ClearHistory);
    }

    pub fn stop(&mut self) {
        let _ = self.commands.send(Command::Stop);
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}

impl Drop for Sampler {
    fn drop(&mut self) {
        self.stop();
    }
}

fn run(mut inner: Inner, initial: Option<Duration>, tx: Sender<Snapshot>, rx: Receiver<Command>) {
    let mut interval = initial;

    let _ = tx.send(inner.sample());

    loop {
        let wait = interval.unwrap_or(Duration::from_millis(200));
        match rx.recv_timeout(wait) {
            Ok(Command::SetInterval(next)) => {
                interval = next;
                let _ = tx.send(inner.sample());
            }
            Ok(Command::RefreshNow) => {
                let _ = tx.send(inner.sample());
            }
            Ok(Command::ClearHistory) => {
                inner.history.clear();
                let _ = inner.history.flush();
                let _ = tx.send(inner.sample());
            }
            Ok(Command::Stop) => break,
            Err(RecvTimeoutError::Timeout) => {
                if interval.is_some() {
                    let _ = tx.send(inner.sample());
                }
            }
            Err(RecvTimeoutError::Disconnected) => break,
        }
    }

    if let Err(err) = inner.history.flush() {
        eprintln!("taskmgr: could not save history: {err}");
    }
}

struct Inner {
    sys: System,
    disks: Disks,
    networks: Networks,
    users: Users,
    classifier: Classifier,
    history: History,
    runner: Box<dyn Runner + Send>,
    last: Instant,
    last_flush: Instant,
    services: Vec<crate::model::ServiceRow>,
    services_error: Option<String>,
    startup: Vec<crate::model::StartupRow>,
    startup_error: Option<String>,
    services_at: Option<Instant>,
    sampled: bool,
    disk_counters: HashMap<String, (u64, u64)>,
    net_counters: HashMap<String, (u64, u64)>,
}

impl Inner {
    fn new() -> Result<Self, String> {
        let mut sys =
            System::new().map_err(|err| format!("cannot read system information: {err}"))?;
        sys.refresh_cpu_all();
        sys.refresh_memory();
        Ok(Self {
            sys,
            disks: Disks::new_with_refreshed_list()
                .map_err(|err| format!("cannot read disks: {err}"))?,
            networks: Networks::new_with_refreshed_list()
                .map_err(|err| format!("cannot read networks: {err}"))?,
            users: Users::new_with_refreshed_list()
                .map_err(|err| format!("cannot read users: {err}"))?,
            classifier: Classifier::new(),
            history: History::load(),
            runner: Box::new(RealRunner),
            last: Instant::now(),
            last_flush: Instant::now(),
            services: Vec::new(),
            services_error: None,
            startup: Vec::new(),
            startup_error: None,
            services_at: None,
            sampled: false,
            disk_counters: HashMap::new(),
            net_counters: HashMap::new(),
        })
    }

    fn sample(&mut self) -> Snapshot {
        let now = Instant::now();
        let dt = if self.sampled {
            now.duration_since(self.last)
        } else {
            Duration::ZERO
        };
        self.sampled = true;
        self.last = now;
        let dt_secs = dt.as_secs_f64();
        let mut errors = Vec::new();

        self.sys.refresh_cpu_all();
        self.sys.refresh_memory();
        self.sys.refresh_processes_specifics(
            ProcessesToUpdate::All,
            true,
            ProcessRefreshKind::nothing()
                .with_cpu()
                .with_memory()
                .with_disk_usage()
                .with_user(UpdateKind::OnlyIfNotSet)
                .with_exe(UpdateKind::OnlyIfNotSet)
                .with_cmd(UpdateKind::OnlyIfNotSet)
                .with_tasks(),
        );
        self.disks.refresh(true);
        self.networks.refresh(true);

        let logical = self.sys.cpus().len().max(1);
        let cpu = self.cpu_stat(logical);
        let memory = self.mem_stat();
        let disks = self.disk_stats(dt_secs);
        let networks = self.network_stats(dt_secs);
        let processes = self.process_rows(logical, dt_secs);
        let mut sessions = self.session_rows(&processes);

        let services_due = self
            .services_at
            .is_none_or(|at| now.duration_since(at) >= SERVICES_TTL);
        if services_due || self.services_error.is_some() {
            self.services_at = Some(now);
            match platform::list_services(self.runner.as_ref()) {
                Ok(rows) => {
                    self.services = rows;
                    self.services_error = None;
                }
                Err(err) => self.services_error = Some(err),
            }
            match platform::list_startup(self.runner.as_ref()) {
                Ok(rows) => {
                    self.startup = rows;
                    self.startup_error = None;
                }
                Err(err) => self.startup_error = Some(err),
            }
        }

        let uptime_secs = System::uptime().unwrap_or(0);
        let known_threads: usize = processes.iter().filter_map(|p| p.threads).sum();
        let thread_count = processes
            .iter()
            .any(|p| p.threads.is_some())
            .then_some(known_threads);

        let mut snapshot = Snapshot {
            cpu,
            memory,
            disks,
            networks,
            gpus: Vec::<GpuStat>::new(),
            processes,
            sessions: std::mem::take(&mut sessions),
            services: self.services.clone(),
            startup: self.startup.clone(),
            uptime_secs,
            thread_count,
            handle_count: handle_count(),
            caps: Some(Capabilities::for_current_platform()),
            errors: std::mem::take(&mut errors),
            services_error: self.services_error.clone(),
            startup_error: self.startup_error.clone(),
            app_usage: Vec::new(),
            history_error: self.history.warning.clone(),
        };

        if dt_secs > 0.0 {
            self.history
                .record(&snapshot, dt, logical, crate::store::history::now_epoch());
        }
        snapshot.app_usage = self.history.rows();

        if self.last_flush.elapsed() >= HISTORY_FLUSH_EVERY {
            let _ = self.history.flush();
            self.last_flush = Instant::now();
        }

        snapshot
    }

    fn cpu_stat(&self, logical: usize) -> CpuStat {
        let per_core: Vec<f32> = self.sys.cpus().iter().map(|cpu| cpu.usage()).collect();
        let first = self.sys.cpus().first();
        CpuStat {
            total: self.sys.global_cpu_usage(),
            per_core,
            speed_mhz: first.map(|c| c.frequency()).unwrap_or(0),
            brand: first.map(|c| c.brand().to_string()).unwrap_or_default(),
            logical,
            physical: System::physical_core_count().ok(),
        }
    }

    fn mem_stat(&self) -> MemStat {
        MemStat {
            total: self.sys.total_memory(),
            used: self.sys.used_memory(),
            available: self.sys.available_memory(),
            swap_total: self.sys.total_swap(),
            swap_used: self.sys.used_swap(),
            compressed: None,
            cached: None,
        }
    }

    fn disk_stats(&mut self, dt_secs: f64) -> Vec<DiskStat> {
        let mut rows = Vec::new();
        for disk in self.disks.iter() {
            let name = disk.name().to_string_lossy().to_string();
            let usage = disk.usage();
            let previous = self.disk_counters.insert(
                name.clone(),
                (usage.total_read_bytes, usage.total_written_bytes),
            );
            let (read, written) =
                previous.unwrap_or((usage.total_read_bytes, usage.total_written_bytes));
            rows.push(DiskStat {
                name,
                read_bps: delta_rate(read, usage.total_read_bytes, dt_secs),
                write_bps: delta_rate(written, usage.total_written_bytes, dt_secs),
                busy: 0.0,
                total: disk.total_space(),
                available: disk.available_space(),
            });
        }
        rows
    }

    fn network_stats(&mut self, dt_secs: f64) -> Vec<NetStat> {
        let mut rows = Vec::new();
        for (name, data) in self.networks.iter() {
            let name = name.to_string_lossy().to_string();
            let previous = self.net_counters.insert(
                name.clone(),
                (data.total_received(), data.total_transmitted()),
            );
            let (rx, tx) = previous.unwrap_or((data.total_received(), data.total_transmitted()));
            rows.push(NetStat {
                name,
                rx_bps: delta_rate(rx, data.total_received(), dt_secs),
                tx_bps: delta_rate(tx, data.total_transmitted(), dt_secs),
                link_speed: None,
            });
        }
        rows
    }

    fn process_rows(&self, logical: usize, dt_secs: f64) -> Vec<ProcRow> {
        let names: HashMap<u32, String> = self
            .users
            .iter()
            .map(|user| (**user.id(), user.name().to_string()))
            .collect();

        self.sys
            .processes()
            .iter()
            .map(|(pid, process)| {
                let uid = process.user_id().map(|id| **id);
                let exe = process.exe().map(|path| path.to_string_lossy().to_string());
                let name = process.name().to_string_lossy().to_string();
                let disk = process.disk_usage();
                let divide = if dt_secs > 0.0 { dt_secs } else { 1.0 };
                ProcRow {
                    key: ProcKey::new(pid.as_u32(), process.start_time()),
                    name: name.clone(),
                    kind: self
                        .classifier
                        .classify(&name, exe.as_deref(), uid, pid.as_u32()),
                    status: map_status(process.status()),
                    cpu: process.cpu_usage() / logical as f32,
                    memory: process.memory(),
                    disk: DiskIo {
                        read_bps: if dt_secs > 0.0 {
                            disk.read_bytes as f64 / divide
                        } else {
                            0.0
                        },
                        write_bps: if dt_secs > 0.0 {
                            disk.written_bytes as f64 / divide
                        } else {
                            0.0
                        },
                    },
                    network: None,
                    gpu: None,
                    user: uid.and_then(|id| names.get(&id).cloned()),
                    uid,
                    parent_pid: process.parent().map(Pid::as_u32),
                    exe,
                    cmd: process
                        .cmd()
                        .iter()
                        .map(|part| part.to_string_lossy())
                        .collect::<Vec<_>>()
                        .join(" "),
                    threads: process.tasks().map(|tasks| tasks.len()),
                    priority: Priority {
                        nice: nice_of(pid.as_u32()),
                    },
                    accumulated_cpu_ms: process.accumulated_cpu_time(),
                    run_time_secs: process.run_time(),
                }
            })
            .collect()
    }

    fn session_rows(&self, processes: &[ProcRow]) -> Vec<SessionRow> {
        let mut sessions = platform::list_sessions(self.runner.as_ref()).unwrap_or_default();

        let mut totals: HashMap<u32, (f32, u64, DiskIo)> = HashMap::new();
        for process in processes {
            if let Some(uid) = process.uid {
                let entry = totals.entry(uid).or_default();
                entry.0 += process.cpu;
                entry.1 += process.memory;
                entry.2.read_bps += process.disk.read_bps;
                entry.2.write_bps += process.disk.write_bps;
            }
        }

        for session in &mut sessions {
            if let Some((cpu, memory, disk)) = totals.get(&session.uid) {
                session.cpu = *cpu;
                session.memory = *memory;
                session.disk = *disk;
            }
        }
        sessions
    }
}

pub fn map_status(status: ProcessStatus) -> Status {
    match status {
        ProcessStatus::Run => Status::Running,
        ProcessStatus::Sleep | ProcessStatus::Idle => Status::Sleeping,
        ProcessStatus::Stop | ProcessStatus::Suspended => Status::Suspended,
        ProcessStatus::Zombie | ProcessStatus::Dead => Status::Stopped,
        ProcessStatus::UninterruptibleDiskSleep | ProcessStatus::LockBlocked => {
            Status::NotResponding
        }
        _ => Status::Sleeping,
    }
}

fn nice_of(pid: u32) -> i32 {
    // SAFETY: getpriority has no preconditions. It reports -1 both as a valid
    // value and on error, so an error reads as normal priority.
    let value = unsafe { libc::getpriority(libc::PRIO_PROCESS, pid as libc::id_t) };
    if (-20..=19).contains(&value) {
        value
    } else {
        0
    }
}

fn handle_count() -> Option<usize> {
    #[cfg(target_os = "linux")]
    {
        let text = std::fs::read_to_string("/proc/sys/fs/file-nr").ok()?;
        let used: usize = text.split_whitespace().next()?.parse().ok()?;
        Some(used)
    }
    #[cfg(not(target_os = "linux"))]
    {
        None
    }
}

/// Normalise a per-core percentage to a share of the whole machine.
pub fn normalise_cpu(per_core_percent: f32, logical: usize) -> f32 {
    per_core_percent / logical.max(1) as f32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_status_maps_to_a_windows_label() {
        assert_eq!(map_status(ProcessStatus::Run), Status::Running);
        assert_eq!(map_status(ProcessStatus::Sleep), Status::Sleeping);
        assert_eq!(map_status(ProcessStatus::Idle), Status::Sleeping);
        assert_eq!(map_status(ProcessStatus::Stop), Status::Suspended);
        assert_eq!(map_status(ProcessStatus::Suspended), Status::Suspended);
        assert_eq!(map_status(ProcessStatus::Zombie), Status::Stopped);
        assert_eq!(map_status(ProcessStatus::Dead), Status::Stopped);
        assert_eq!(
            map_status(ProcessStatus::UninterruptibleDiskSleep),
            Status::NotResponding
        );
        assert_eq!(
            map_status(ProcessStatus::LockBlocked),
            Status::NotResponding
        );
        assert_eq!(map_status(ProcessStatus::Tracing), Status::Sleeping);
        assert_eq!(map_status(ProcessStatus::Unknown(7)), Status::Sleeping);
    }

    #[test]
    fn delta_rate_handles_first_sample_and_resets() {
        assert_eq!(delta_rate(0, 0, 0.0), 0.0);
        assert_eq!(delta_rate(100, 200, 1.0), 100.0);
        assert_eq!(delta_rate(100, 200, 2.0), 50.0);
        assert_eq!(delta_rate(500, 100, 1.0), 0.0, "counter reset reads zero");
        assert_eq!(delta_rate(500, 100, -1.0), 0.0);
    }

    #[test]
    fn cpu_normalisation_matches_windows() {
        assert_eq!(normalise_cpu(100.0, 8), 12.5);
        assert_eq!(normalise_cpu(800.0, 8), 100.0);
        assert_eq!(normalise_cpu(50.0, 1), 50.0);
        assert_eq!(normalise_cpu(50.0, 0), 50.0);
    }

    #[test]
    fn sampler_produces_a_snapshot_and_stops() {
        let mut sampler = Sampler::spawn(Some(Duration::from_millis(50))).unwrap();
        let first = sampler
            .snapshots
            .recv_timeout(Duration::from_secs(5))
            .expect("first snapshot");
        assert!(first.cpu.logical >= 1);
        assert!(!first.processes.is_empty());
        // The first sample has no elapsed time, so rates are zero.
        assert!(first.disks.iter().all(|d| d.read_bps == 0.0));
        sampler.stop();
    }

    #[test]
    fn pause_stops_snapshots_and_resume_refreshes() {
        let mut sampler = Sampler::spawn(Some(Duration::from_millis(50))).unwrap();
        let _ = sampler
            .snapshots
            .recv_timeout(Duration::from_secs(5))
            .unwrap();
        sampler.set_interval(None);
        // Drain the sample produced by the interval change.
        let _ = sampler.snapshots.recv_timeout(Duration::from_secs(2));
        assert!(
            sampler
                .snapshots
                .recv_timeout(Duration::from_millis(300))
                .is_err(),
            "paused sampler must not produce snapshots"
        );
        sampler.set_interval(Some(Duration::from_millis(50)));
        assert!(
            sampler
                .snapshots
                .recv_timeout(Duration::from_secs(5))
                .is_ok(),
            "resume must refresh immediately"
        );
        sampler.stop();
    }
}
