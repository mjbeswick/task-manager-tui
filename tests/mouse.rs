//! Mouse targets are recorded by the frame itself. These tests render a frame
//! and then click the regions it published.

use task_manager_tui::app::{App, Dialog, PendingAction, Tab};
use task_manager_tui::store::settings::Settings;
use task_manager_tui::ui::hits::{HitMap, Target};
use task_manager_tui::ui::test_util::render_with_hits;

fn app(tab: Tab) -> App {
    let (mut app, _) = App::new(Settings::default(), false);
    app.apply_snapshot(task_manager_tui::model::Snapshot::fixture());
    app.tab = tab;
    app
}

fn targets(hits: &HitMap, width: u16, height: u16) -> Vec<Target> {
    let mut found = Vec::new();
    for y in 0..height {
        for x in 0..width {
            if let Some(target) = hits.hit(x, y)
                && !found.contains(&target)
            {
                found.push(target);
            }
        }
    }
    found
}

#[test]
fn chrome_publishes_menu_tabs_and_rows() {
    let app = app(Tab::Processes);
    let (_, hits) = render_with_hits(&app, 120, 40);
    let found = targets(&hits, 120, 40);
    assert!(found.contains(&Target::Menu(0)), "File menu");
    assert!(found.contains(&Target::Tab(Tab::Performance)));
    assert!(found.contains(&Target::Tab(Tab::Services)));
    assert!(found.contains(&Target::Row(0)));
    assert!(found.contains(&Target::Row(5)));
    assert!(found.contains(&Target::FewerDetails));
}

#[test]
fn performance_rail_is_clickable() {
    let app = app(Tab::Performance);
    let (_, hits) = render_with_hits(&app, 120, 40);
    let found = targets(&hits, 120, 40);
    for resource in [
        task_manager_tui::app::ResourceChoice::Cpu,
        task_manager_tui::app::ResourceChoice::Memory,
        task_manager_tui::app::ResourceChoice::Gpu,
    ] {
        assert!(found.contains(&Target::Resource(resource)), "{resource:?}");
    }
}

#[test]
fn rail_sparkline_rows_select_their_resource() {
    let app = app(Tab::Performance);
    let (screen, hits) = render_with_hits(&app, 120, 36);
    let row = screen
        .lines()
        .position(|line| line.starts_with("  Memory"))
        .expect("Memory entry") as u16;
    assert_eq!(
        hits.hit(5, row + 1),
        Some(Target::Resource(
            task_manager_tui::app::ResourceChoice::Memory
        )),
        "the sparkline row under Memory"
    );
}

#[test]
fn open_menu_publishes_its_items() {
    let mut app = app(Tab::Processes);
    app.menu = Some(task_manager_tui::app::MenuState { menu: 2, item: 0 });
    let (_, hits) = render_with_hits(&app, 120, 40);
    let found = targets(&hits, 120, 40);
    assert!(found.contains(&Target::MenuItem(2, 0)));
    assert!(found.contains(&Target::MenuItem(2, 5)));
}

#[test]
fn confirm_dialog_publishes_both_buttons() {
    let mut app = app(Tab::Processes);
    app.dialog = Some(Dialog::Confirm {
        title: "End task".into(),
        message: "End Ghostty?".into(),
        confirm_label: "End task".into(),
        action: PendingAction::ClearHistory,
    });
    let (_, hits) = render_with_hits(&app, 120, 40);
    let found = targets(&hits, 120, 40);
    assert!(found.contains(&Target::DialogConfirm));
    assert!(found.contains(&Target::DialogCancel));
}

#[test]
fn priority_dialog_publishes_each_level() {
    let mut app = app(Tab::Details);
    let row = app.snapshot.processes[0].clone();
    app.dialog = Some(Dialog::Priority {
        key: row.key,
        name: row.name,
        selected: 0,
    });
    let (_, hits) = render_with_hits(&app, 120, 40);
    let found = targets(&hits, 120, 40);
    for index in 0..5 {
        assert!(
            found.contains(&Target::DialogPriority(index)),
            "level {index}"
        );
    }
    assert!(found.contains(&Target::DialogConfirm));
}
