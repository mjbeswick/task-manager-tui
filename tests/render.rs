//! Golden frames for every tab, both themes, three sizes, and the dialogs.

use task_manager_tui::app::{App, Dialog, PendingAction, ResourceChoice, Tab};
use task_manager_tui::model::Snapshot;
use task_manager_tui::store::settings::Settings;
use task_manager_tui::ui::test_util::render;

fn fixture() -> Snapshot {
    let mut snapshot = Snapshot::fixture();
    let now = task_manager_tui::store::history::now_epoch();
    for row in &mut snapshot.app_usage {
        row.last_seen = now - 3_600;
    }
    snapshot
}

fn app(tab: Tab, theme: &str) -> App {
    let settings = Settings {
        theme: theme.to_string(),
        ..Settings::default()
    };
    let (mut app, _) = App::new(settings, false);
    // A full history so the charts span their plot.
    for _ in 0..60 {
        app.apply_snapshot(fixture());
    }
    app.tab = tab;
    app
}

fn screen(app: &App, width: u16, height: u16) -> String {
    render(app, width, height)
}

#[test]
fn golden_all_tabs_light() {
    for tab in Tab::ALL {
        insta::assert_snapshot!(
            format!("light-{}-120x40", slug(tab)),
            screen(&app(tab, "light"), 120, 40)
        );
    }
}

#[test]
fn golden_all_tabs_dark() {
    for tab in Tab::ALL {
        insta::assert_snapshot!(
            format!("dark-{}-120x40", slug(tab)),
            screen(&app(tab, "dark"), 120, 40)
        );
    }
}

#[test]
fn golden_narrow_frames() {
    insta::assert_snapshot!(
        "light-processes-80x24",
        screen(&app(Tab::Processes, "light"), 80, 24)
    );
    insta::assert_snapshot!(
        "light-performance-80x24",
        screen(&app(Tab::Performance, "light"), 80, 24)
    );
    insta::assert_snapshot!(
        "light-details-80x24",
        screen(&app(Tab::Details, "light"), 80, 24)
    );
    insta::assert_snapshot!(
        "light-processes-40x20",
        screen(&app(Tab::Processes, "light"), 40, 20)
    );
    insta::assert_snapshot!(
        "below-minimum-30x8",
        screen(&app(Tab::Processes, "light"), 30, 8)
    );
}

#[test]
fn golden_performance_resources() {
    for (resource, name) in [
        (ResourceChoice::Memory, "memory"),
        (ResourceChoice::Disk, "disk"),
        (ResourceChoice::Network, "network"),
        (ResourceChoice::Gpu, "gpu"),
    ] {
        let mut app = app(Tab::Performance, "light");
        app.performance.resource = resource;
        insta::assert_snapshot!(
            format!("light-performance-{name}-120x40"),
            screen(&app, 120, 40)
        );
    }
}

#[test]
fn golden_chart_style_and_colour() {
    let mut app = app(Tab::Performance, "light");
    app.set_chart_style(task_manager_tui::model::ChartStyle::Line);
    app.set_chart_color(task_manager_tui::model::ChartColor::Blue);
    insta::assert_snapshot!("light-performance-line-120x40", screen(&app, 120, 40));
}

#[test]
fn golden_appearance_dialog() {
    let mut app = app(Tab::Performance, "light");
    app.dialog = Some(Dialog::Appearance { selected: 1 });
    insta::assert_snapshot!("dialog-appearance-120x40", screen(&app, 120, 40));
}

#[test]
fn golden_fewer_details() {
    let mut app = app(Tab::Processes, "light");
    app.process.more_details = false;
    insta::assert_snapshot!("light-processes-fewer-120x40", screen(&app, 120, 40));
}

#[test]
fn golden_details_panel() {
    let mut app = app(Tab::Details, "light");
    app.details.panel = true;
    insta::assert_snapshot!("light-details-panel-120x40", screen(&app, 120, 40));
}

#[test]
fn golden_priority_dialog() {
    let mut app = app(Tab::Details, "light");
    let row = app.snapshot.processes[4].clone();
    app.dialog = Some(Dialog::Priority {
        key: row.key,
        name: row.name,
        input: "-5".into(),
        editing: false,
        error: false,
    });
    insta::assert_snapshot!("dialog-priority-120x40", screen(&app, 120, 40));
}

#[test]
fn golden_dialogs() {
    let mut app = app(Tab::Processes, "light");
    app.dialog = Some(Dialog::Confirm {
        title: "End task".into(),
        message: "End WindowServer (PID 500)? This process belongs to _windowserver, so ending it may need administrator rights."
            .into(),
        confirm_label: "End task".into(),
        action: PendingAction::ClearHistory,
    });
    insta::assert_snapshot!("dialog-confirm-120x40", screen(&app, 120, 40));

    app.dialog = Some(Dialog::Error {
        title: "Could not end task".into(),
        message: "Operation not permitted for PID 500. It belongs to another user or needs administrator rights."
            .into(),
    });
    insta::assert_snapshot!("dialog-error-120x40", screen(&app, 120, 40));

    app.dialog = Some(Dialog::Help {
        offset: std::cell::Cell::new(0),
    });
    insta::assert_snapshot!("dialog-help-120x40", screen(&app, 120, 40));

    app.dialog = Some(Dialog::Run {
        input: "open -a Calculator".into(),
    });
    insta::assert_snapshot!("dialog-run-120x40", screen(&app, 120, 40));

    app.dialog = Some(Dialog::Properties {
        title: "Properties".into(),
        lines: vec![
            "Name: homebrew.mxcl.colima".into(),
            "Command: /opt/homebrew/opt/colima/bin/colima start --foreground".into(),
            "Owner: michael".into(),
            "Source: /Users/michael/Library/LaunchAgents/homebrew.mxcl.colima.plist".into(),
            "Status: Stopped".into(),
            "Startup impact: Not measured".into(),
        ],
    });
    insta::assert_snapshot!("dialog-properties-120x40", screen(&app, 120, 40));
}

#[test]
fn golden_menu_open() {
    let mut app = app(Tab::Processes, "light");
    app.menu = Some(task_manager_tui::app::MenuState { menu: 1, item: 2 });
    insta::assert_snapshot!("light-menu-options-120x40", screen(&app, 120, 40));
}

fn slug(tab: Tab) -> &'static str {
    match tab {
        Tab::Processes => "processes",
        Tab::Performance => "performance",
        Tab::AppHistory => "app-history",
        Tab::Startup => "startup",
        Tab::Users => "users",
        Tab::Details => "details",
        Tab::Services => "services",
    }
}
