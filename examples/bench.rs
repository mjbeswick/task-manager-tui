use std::time::Instant;
use sysinfo::{Disks, Networks, ProcessRefreshKind, ProcessesToUpdate, System, UpdateKind};

fn main() {
    let mut sys = System::new().unwrap();
    let mut disks = Disks::new_with_refreshed_list().unwrap();
    let mut nets = Networks::new_with_refreshed_list().unwrap();
    let kind = ProcessRefreshKind::nothing()
        .with_cpu()
        .with_memory()
        .with_disk_usage()
        .with_user(UpdateKind::OnlyIfNotSet)
        .with_exe(UpdateKind::OnlyIfNotSet)
        .with_cmd(UpdateKind::OnlyIfNotSet)
        .with_tasks();
    let mut worst = 0u128;
    for _ in 0..5 {
        let start = Instant::now();
        sys.refresh_cpu_all();
        sys.refresh_memory();
        sys.refresh_processes_specifics(ProcessesToUpdate::All, true, kind);
        disks.refresh(true);
        nets.refresh(true);
        let elapsed = start.elapsed().as_millis();
        worst = worst.max(elapsed);
        std::thread::sleep(std::time::Duration::from_millis(200));
    }
    println!(
        "processes={} worst_refresh={}ms",
        sys.processes().len(),
        worst
    );
}
