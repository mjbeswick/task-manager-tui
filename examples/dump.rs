use task_manager_tui::app::{App, Tab};
use task_manager_tui::store::settings::Settings;
use task_manager_tui::ui::test_util::render;

fn main() {
    let (mut app, _) = App::new(Settings::default(), false);
    app.apply_snapshot(task_manager_tui::model::Snapshot::fixture());
    let tab = std::env::args().nth(1).unwrap_or_default();
    app.tab = match tab.as_str() {
        "performance" => Tab::Performance,
        "details" => Tab::Details,
        "users" => Tab::Users,
        "app-history" => Tab::AppHistory,
        "startup" => Tab::Startup,
        "services" => Tab::Services,
        _ => Tab::Processes,
    };
    println!("{}", render(&app, 120, 40));
}
