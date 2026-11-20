//! Render demo frames as HTML for the README screenshots.
//!
//! Usage: cargo run --example screenshot -- <tab> <theme> <out.html> [width] [height]
//!
//! The frame uses `Snapshot::fixture` plus a few extra generic processes, so no
//! real machine data ends up in the images.

use ratatui::Terminal;
use ratatui::backend::TestBackend;
use ratatui::style::Color;
use task_manager_tui::app::{App, ResourceChoice, Tab};
use task_manager_tui::model::{DiskIo, Kind, Priority, ProcKey, ProcRow, Snapshot, Status};
use task_manager_tui::store::settings::Settings;
use task_manager_tui::theme::ThemeName;
use task_manager_tui::ui::hits::HitMap;

fn main() {
    let mut args = std::env::args().skip(1);
    let tab = match args.next().unwrap_or_default().as_str() {
        "processes" => Tab::Processes,
        "performance" => Tab::Performance,
        "details" => Tab::Details,
        "services" => Tab::Services,
        other => panic!("unknown tab {other}"),
    };
    let theme = ThemeName::parse(&args.next().unwrap_or_else(|| "dark".into())).unwrap();
    let out = args.next().expect("output path");
    let width: u16 = args
        .next()
        .and_then(|value| value.parse().ok())
        .unwrap_or(120);
    let height: u16 = args
        .next()
        .and_then(|value| value.parse().ok())
        .unwrap_or(36);

    let mut app = App::new(Settings::default(), false).0;
    app.apply_snapshot(demo_snapshot());
    app.set_theme(theme);
    app.tab = tab;
    match tab {
        Tab::Performance => {
            app.performance.resource = ResourceChoice::Cpu;
            fill_graphs(&mut app);
        }
        Tab::Services => app.sync_services_public(),
        _ => {}
    }

    let mut hits = HitMap::default();
    let mut terminal = Terminal::new(TestBackend::new(width, height)).unwrap();
    terminal
        .draw(|frame| task_manager_tui::ui::draw(frame, &app, &mut hits))
        .unwrap();
    let buffer = terminal.backend().buffer();

    let mut html = String::new();
    html.push_str(
        "<!doctype html><meta charset=\"utf-8\"><style>\
         html,body{margin:0;padding:0;background:#0b0b0b;}\
         pre{display:inline-block;font-family:Menlo,DejaVu Sans Mono,monospace;\
         font-size:14px;line-height:1.15;margin:0;padding:12px 16px 20px;white-space:pre;}\
         div{height:1.15em;}</style><pre>",
    );
    for y in 0..height {
        html.push_str("<div>");
        for x in 0..width {
            let cell = &buffer[(x, y)];
            let fg = css(cell.fg);
            let bg = css(cell.bg);
            let text = escape(cell.symbol());
            html.push_str(&format!(
                "<span style=\"color:{fg};background:{bg}\">{text}</span>"
            ));
        }
        html.push_str("</div>");
    }
    html.push_str("</pre>");
    std::fs::write(&out, html).unwrap();
}

fn demo_snapshot() -> Snapshot {
    let mut snapshot = Snapshot::fixture();
    let extra = [
        ("Xcode", Kind::App, 3.2, 900),
        ("Music", Kind::App, 0.4, 120),
        ("Notes", Kind::App, 0.1, 84),
        ("Docker Desktop", Kind::App, 1.7, 430),
        ("mdworker_shared", Kind::Background, 2.6, 140),
        ("backupd", Kind::Background, 0.8, 64),
        ("com.docker.backend", Kind::Background, 1.4, 210),
        ("spotlightd", Kind::Background, 0.3, 48),
        ("kernel_task", Kind::System, 5.1, 320),
        ("mds_stores", Kind::System, 1.9, 150),
        ("coreaudiod", Kind::System, 0.2, 22),
    ];
    for (index, (name, kind, cpu, mem_mb)) in extra.into_iter().enumerate() {
        let pid = 5_000 + index as u32;
        snapshot.processes.push(ProcRow {
            key: ProcKey::new(pid, 1_700_000_000 + pid as u64),
            name: name.to_string(),
            kind,
            status: Status::Running,
            cpu,
            memory: mem_mb * 1024 * 1024,
            disk: DiskIo {
                read_bps: 1024.0 * 4.0,
                write_bps: 1024.0 * 1.5,
            },
            network: None,
            gpu: None,
            user: Some("michael".into()),
            uid: Some(501),
            parent_pid: Some(1),
            exe: Some(format!("/usr/local/bin/{name}")),
            cmd: format!("{name} --flag"),
            threads: Some(3),
            priority: Priority { nice: 0 },
            accumulated_cpu_ms: 12_000,
            run_time_secs: 900,
        });
    }
    snapshot
}

fn fill_graphs(app: &mut App) {
    for i in 0..70 {
        let wave = (i as f32 * 0.35).sin();
        app.graphs.cpu.push(38.0 + wave * 14.0);
        app.graphs.memory.push(62.0 + wave * 4.0);
        app.graphs.disk.push(2_000_000.0 + wave * 800_000.0);
        app.graphs.network.push(4_000_000.0 + wave * 1_500_000.0);
        app.graphs.gpu.push(12.0 + wave * 6.0);
        for core in &mut app.graphs.cores {
            core.push(30.0 + wave * 20.0);
        }
    }
}

fn css(color: Color) -> String {
    match color {
        Color::Reset => "transparent".to_string(),
        other => {
            let (r, g, b) = task_manager_tui::theme::indexed_rgb(other);
            format!("#{r:02x}{g:02x}{b:02x}")
        }
    }
}

fn escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}
