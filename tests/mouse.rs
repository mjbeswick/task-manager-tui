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

/// Click the header cell whose label is `label` and return the new frame.
fn click_header(app: &mut App, label: &str) -> String {
    let (screen, hits) = render_with_hits(app, 120, 40);
    let (y, line) = screen
        .lines()
        .enumerate()
        .find(|(_, l)| l.starts_with("Name"))
        .expect("header row");
    let x = line[..line.find(label).expect(label)].chars().count() as u16;
    let target = hits.hit(x, y as u16).expect("header target");
    assert!(matches!(target, Target::Header(_)), "{target:?}");
    app.click(target);
    render_with_hits(app, 120, 40).0
}

fn header_line(screen: &str) -> String {
    screen
        .lines()
        .find(|l| l.starts_with("Name"))
        .unwrap()
        .to_string()
}

#[test]
fn clicking_a_header_sorts_and_clicking_again_reverses() {
    use task_manager_tui::sample::group::{SortDir, SortKey};
    for tab in [Tab::Processes, Tab::Details] {
        let mut app = app(tab);
        let screen = click_header(&mut app, "Memory");
        let sort = if tab == Tab::Processes {
            app.process.sort
        } else {
            app.details.sort
        };
        assert_eq!(
            (sort.key, sort.dir),
            (SortKey::Memory, SortDir::Desc),
            "{tab:?}"
        );
        assert!(header_line(&screen).contains("Memory ▼"), "{tab:?}");
        let screen = click_header(&mut app, "Memory");
        let sort = if tab == Tab::Processes {
            app.process.sort
        } else {
            app.details.sort
        };
        assert_eq!(sort.dir, SortDir::Asc, "{tab:?}");
        assert!(header_line(&screen).contains("Memory ▲"), "{tab:?}");
        // Lowest memory first: postgres (96 MB) now comes before node (512 MB).
        let postgres = screen.find("postgres").unwrap();
        let node = screen.find("node").unwrap();
        assert!(postgres < node, "{tab:?} order after reversing");
    }
}

#[test]
fn clicking_the_first_row_under_the_two_row_header_selects_it() {
    let mut app = app(Tab::Processes);
    let (screen, hits) = render_with_hits(&app, 120, 40);
    let y = screen.lines().position(|l| l.contains("Apps (")).unwrap() as u16;
    let target = hits.hit(5, y).expect("row target");
    assert_eq!(target, Target::Row(0));
    let y = y + 1;
    let target = hits.hit(5, y).unwrap();
    app.click(target);
    let (screen, _) = render_with_hits(&app, 120, 40);
    assert!(screen.lines().nth(y as usize).unwrap().contains("Safari"));
    assert_eq!(app.process.selected, 1);
}

#[test]
fn clicking_the_filter_field_focuses_it_and_the_clear_mark_empties_it() {
    let mut app = app(Tab::Services);
    let (screen, hits) = render_with_hits(&app, 120, 40);
    let x = screen.lines().next().unwrap().find("/ Filter").unwrap() as u16;
    let target = hits.hit(x + 3, 0).expect("field target");
    assert_eq!(target, Target::FilterField);
    app.click(target);
    assert!(app.filter_editing);
    app.filter_push('n');
    app.filter_editing = false;
    let (_, hits) = render_with_hits(&app, 120, 40);
    assert_eq!(hits.hit(118, 0), Some(Target::FilterClear));
    app.click(Target::FilterClear);
    assert!(app.filter().is_empty());
}
