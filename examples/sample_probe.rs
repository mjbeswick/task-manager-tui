use std::time::Duration;
use task_manager_tui::sample::Sampler;
fn main() {
    let mut s = Sampler::spawn(Some(Duration::from_millis(600))).unwrap();
    for i in 0..3 {
        let snap = s.snapshots.recv_timeout(Duration::from_secs(5)).unwrap();
        let mut rows: Vec<_> = snap.processes.iter().collect();
        rows.sort_by(|a, b| b.cpu.total_cmp(&a.cpu));
        println!(
            "--- sample {i} logical={} total={:.1}",
            snap.cpu.logical, snap.cpu.total
        );
        for p in rows.iter().take(5) {
            println!(
                "  {:<24} cpu={:.2} disk={:.0}",
                p.name,
                p.cpu,
                p.disk.total_bps()
            );
        }
        println!(
            "  disks={:?}",
            snap.disks
                .iter()
                .map(|d| (d.name.clone(), d.read_bps))
                .collect::<Vec<_>>()
        );
    }
    s.stop();
}
