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
}

fn menu_app() -> App {
    let mut app = app(Tab::Processes);
    app.menu = Some(task_manager_tui::app::MenuState { menu: 0, item: 0 });
    app
}

#[test]
fn dropdown_publishes_a_panel_and_its_items() {
    let app = menu_app();
    let (_, hits) = render_with_hits(&app, 120, 40);
    assert!(hits.rect_of(Target::MenuPanel).is_some(), "panel target");
    assert!(
        hits.rect_of(Target::MenuItem(0, 0)).is_some(),
        "first item wins over the panel"
    );
    let panel = hits.rect_of(Target::MenuPanel).unwrap();
    let item = hits.rect_of(Target::MenuItem(0, 0)).unwrap();
    assert_eq!(hits.hit(item.x, item.y), Some(Target::MenuItem(0, 0)));
    assert_eq!(hits.hit(panel.x, panel.y), Some(Target::MenuPanel));
    assert_ne!(
        hits.hit(0, 39),
        Some(Target::MenuPanel),
        "outside is outside"
    );
}

#[test]
fn click_away_closes_the_menu_without_acting() {
    let mut app = menu_app();
    let selected = app.process.selected;
    let effects = app.mouse_click(Some(Target::Row(3)));
    assert!(effects.is_empty());
    assert!(app.menu.is_none());
    assert_eq!(app.process.selected, selected, "row click was consumed");
}

#[test]
fn click_away_does_not_switch_a_tab_sort_or_filter() {
    let mut tab = menu_app();
    tab.mouse_click(Some(Target::Tab(Tab::Performance)));
    assert!(tab.menu.is_none());
    assert_eq!(tab.tab, Tab::Processes);

    let mut sort = menu_app();
    let before = sort.process.sort.key;
    sort.mouse_click(Some(Target::Header(
        task_manager_tui::sample::group::SortKey::Memory,
    )));
    assert!(sort.menu.is_none());
    assert_eq!(sort.process.sort.key, before);

    let mut filter = menu_app();
    filter.mouse_click(Some(Target::FilterField));
    assert!(filter.menu.is_none());
    assert!(!filter.filter_editing);
}

#[test]
fn clicking_inside_the_panel_keeps_the_menu_open() {
    let mut app = menu_app();
    let effects = app.mouse_click(Some(Target::MenuPanel));
    assert!(effects.is_empty());
    assert_eq!(app.menu.unwrap().menu, 0);
    app.mouse_click(Some(Target::Menu(2)));
    assert_eq!(app.menu.unwrap().menu, 2);
}

#[test]
fn click_with_no_target_closes_the_menu() {
    let mut app = menu_app();
    let effects = app.mouse_click(None);
    assert!(effects.is_empty());
    assert!(app.menu.is_none());
}

#[test]
fn scrollbar_and_wheel_zones_are_published() {
    let mut app = app(Tab::Processes);
    let mut snapshot = task_manager_tui::model::Snapshot::fixture();
    let template = snapshot.processes[0].clone();
    for i in 0..300u32 {
        let mut row = template.clone();
        row.key = task_manager_tui::model::ProcKey::new(30_000 + i, 1);
        row.name = format!("proc{i:03}");
        row.kind = task_manager_tui::model::Kind::Background;
        snapshot.processes.push(row);
    }
    app.apply_snapshot(snapshot);
    let (_, hits) = render_with_hits(&app, 120, 40);
    assert!(
        hits.rect_of(Target::ScrollZone(Tab::Processes)).is_some(),
        "rows expose a wheel zone"
    );
    let found = targets(&hits, 120, 40);
    assert!(found.contains(&Target::ScrollbarTrack(Tab::Processes)));
    assert!(found.contains(&Target::ScrollbarThumb(Tab::Processes)));
    let thumb = hits
        .rect_of(Target::ScrollbarThumb(Tab::Processes))
        .unwrap();
    let track = hits
        .rect_of(Target::ScrollbarTrack(Tab::Processes))
        .unwrap();
    assert!(thumb.y >= track.y && thumb.y + thumb.height <= track.y + track.height);
    assert_eq!(
        hits.hit(thumb.x, thumb.y),
        Some(Target::ScrollbarThumb(Tab::Processes))
    );
    if thumb.y > track.y {
        assert_eq!(
            hits.hit(track.x, track.y),
            Some(Target::ScrollbarTrack(Tab::Processes))
        );
    }
    let (_, narrow) = render_with_hits(&app, 50, 40);
    assert!(!targets(&narrow, 50, 40).contains(&Target::ScrollbarThumb(Tab::Processes)));
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
    assert!(found.contains(&Target::MenuItem(2, 4)));
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
fn signal_dialog_publishes_each_signal() {
    let mut app = app(Tab::Details);
    let row = app.snapshot.processes[0].clone();
    app.dialog = Some(Dialog::Signal {
        key: row.key,
        name: row.name,
        pid: row.key.pid,
        selected: 0,
        tree: false,
    });
    let (_, hits) = render_with_hits(&app, 120, 40);
    let found = targets(&hits, 120, 40);
    for index in 0..6 {
        assert!(
            found.contains(&Target::DialogSignal(index)),
            "signal {index}"
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
