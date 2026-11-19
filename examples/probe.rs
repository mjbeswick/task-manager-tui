use sysinfo::{ProcessRefreshKind, ProcessesToUpdate, System};
fn main() {
    let mut sys = System::new().unwrap();
    sys.refresh_cpu_all();
    sys.refresh_processes_specifics(
        ProcessesToUpdate::All,
        true,
        ProcessRefreshKind::everything(),
    );
    std::thread::sleep(std::time::Duration::from_millis(600));
    sys.refresh_processes_specifics(
        ProcessesToUpdate::All,
        true,
        ProcessRefreshKind::everything(),
    );
    let mut rows: Vec<_> = sys.processes().iter().collect();
    rows.sort_by_key(|(_, p)| std::cmp::Reverse(p.cpu_usage() as u64));
    for (pid, p) in rows.iter().take(12) {
        println!(
            "{pid:>7} {:<28} {:?} cpu={:.1} exe={:?}",
            p.name().to_string_lossy(),
            p.status(),
            p.cpu_usage(),
            p.exe()
        );
    }
    let sleeping = sys
        .processes()
        .values()
        .filter(|p| matches!(p.status(), sysinfo::ProcessStatus::Sleep))
        .count();
    println!("sleeping={sleeping} total={}", sys.processes().len());
}
