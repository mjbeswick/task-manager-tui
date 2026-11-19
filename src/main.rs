use anyhow::{Context, Result};
use clap::Parser;
use crossterm::event::{
    self, DisableMouseCapture, EnableMouseCapture, Event, MouseButton, MouseEvent, MouseEventKind,
};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::{cursor, execute, terminal};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use std::io::{self, IsTerminal, Stdout, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use task_manager_tui::app::{App, Dialog, Effect};
use task_manager_tui::cli::Cli;
use task_manager_tui::platform::{self, RealRunner};
use task_manager_tui::sample::Sampler;
use task_manager_tui::store::settings;
use task_manager_tui::ui;
use task_manager_tui::ui::hits::{HitMap, Target};

static RESTORED: AtomicBool = AtomicBool::new(false);

fn main() -> Result<()> {
    let cli = Cli::parse();
    let (mut loaded, warning) = settings::load();
    if let Some(theme) = &cli.theme {
        loaded.theme = theme.clone();
    }
    if let Some(interval) = &cli.interval {
        loaded.set_speed(interval);
    }
    if cli.ascii {
        loaded.ascii = true;
    }
    if let Some(tab) = cli.tab {
        loaded.last_tab = tab.index();
    }

    let (mut app, notices) = App::new(loaded, cli.ascii);
    app.set_depth(
        task_manager_tui::theme::ColorDepth::parse(&cli.color)
            .unwrap_or_else(task_manager_tui::theme::ColorDepth::from_env),
    );
    for notice in notices {
        app.notice(notice);
    }
    if let Some(warning) = warning {
        app.notice(warning);
    }

    if cli.once {
        return once(&mut app);
    }

    if !io::stdout().is_terminal() {
        eprintln!("taskmgr needs a terminal. Run it directly, or use --once to print one frame.");
        std::process::exit(1);
    }

    let mut sampler = Sampler::spawn(app.settings.interval())
        .map_err(|err| anyhow::anyhow!(err))
        .context("cannot start the process sampler")?;
    let mut terminal = init_terminal().context("cannot start the terminal")?;

    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        restore_terminal();
        default_hook(info);
    }));

    let result = run(&mut terminal, &mut app, &mut sampler);

    if let Err(err) = settings::save(&app.settings) {
        app.notice(err);
    }
    sampler.stop();
    restore_terminal();
    result
}

fn run(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    app: &mut App,
    sampler: &mut Sampler,
) -> Result<()> {
    let mut hits = HitMap::default();
    let mut last_click: Option<(std::time::Instant, Target)> = None;
    let mut drag: Option<Drag> = None;
    let mut last_tab = app.tab;
    set_title(app.tab);

    loop {
        terminal.draw(|frame| ui::draw(frame, app, &mut hits))?;

        if app.tab != last_tab {
            set_title(app.tab);
            last_tab = app.tab;
        }

        if event::poll(Duration::from_millis(250))? {
            match event::read()? {
                Event::Key(key) => {
                    if let Some(action) = task_manager_tui::input::translate(app, key) {
                        let effects = app.dispatch(action);
                        if apply_effects(app, sampler, effects)? {
                            break;
                        }
                    }
                }
                Event::Mouse(mouse)
                    if handle_mouse(app, sampler, &hits, mouse, &mut last_click, &mut drag)? =>
                {
                    break;
                }
                _ => {}
            }
        }

        while let Ok(snapshot) = sampler.snapshots.try_recv() {
            app.apply_snapshot(snapshot);
        }

        if app.quit {
            break;
        }
    }
    Ok(())
}

/// A scrollbar drag in progress.
struct Drag {
    tab: task_manager_tui::app::Tab,
    grab: u16,
    track_top: u16,
    track: u16,
    thumb: u16,
    max_scroll: usize,
}

/// Handle a mouse event. Returns true when the application should exit.
fn handle_mouse(
    app: &mut App,
    sampler: &mut Sampler,
    hits: &HitMap,
    mouse: MouseEvent,
    last_click: &mut Option<(std::time::Instant, Target)>,
    drag: &mut Option<Drag>,
) -> Result<bool> {
    use task_manager_tui::app::Tab;
    use task_manager_tui::input::Action;

    match mouse.kind {
        MouseEventKind::Drag(MouseButton::Left) => {
            if let Some(state) = drag {
                let viewport = task_manager_tui::ui::chrome::drag_viewport(
                    state.track_top,
                    state.grab,
                    mouse.row,
                    state.track,
                    state.thumb,
                    state.max_scroll,
                );
                app.set_list_scroll(state.tab, viewport);
            }
            Ok(false)
        }
        MouseEventKind::Up(_) => {
            *drag = None;
            Ok(false)
        }
        MouseEventKind::Down(MouseButton::Left) => {
            let target = hits.hit(mouse.column, mouse.row);
            if app.menu.is_some() {
                let effects = app.mouse_click(target);
                return apply_effects(app, sampler, effects);
            }
            let Some(target) = target else {
                return Ok(false);
            };
            if let Target::ScrollbarThumb(tab) = target
                && let Some(thumb) = hits.rect_of(target)
                && let Some(track) = hits.rect_of(Target::ScrollbarTrack(tab))
            {
                let visible = app.list_visible(tab);
                *drag = Some(Drag {
                    tab,
                    grab: mouse.row.saturating_sub(thumb.y),
                    track_top: track.y,
                    track: track.height,
                    thumb: thumb.height,
                    max_scroll: app.list_len(tab).saturating_sub(visible),
                });
                return Ok(false);
            }
            if let Target::ScrollbarTrack(tab) = target
                && let Some(thumb) = hits.rect_of(Target::ScrollbarThumb(tab))
            {
                if mouse.row < thumb.y {
                    app.page_list(tab, -1);
                } else if mouse.row >= thumb.y + thumb.height {
                    app.page_list(tab, 1);
                }
                return Ok(false);
            }
            let now = std::time::Instant::now();
            let double = matches!(target, Target::Row(_))
                && last_click.is_some_and(|(at, previous)| {
                    previous == target && now.duration_since(at) < Duration::from_millis(400)
                });
            *last_click = Some((now, target));

            let mut effects = app.click(target);
            if double {
                effects.extend(app.dispatch(Action::Expand));
                *last_click = None;
            }
            apply_effects(app, sampler, effects)
        }
        MouseEventKind::ScrollDown | MouseEventKind::ScrollUp => {
            let down = matches!(mouse.kind, MouseEventKind::ScrollDown);
            let target = hits.hit(mouse.column, mouse.row);
            match target {
                Some(Target::MenuItem(..)) => {
                    let action = if down {
                        Action::MenuDown
                    } else {
                        Action::MenuUp
                    };
                    let effects = app.dispatch(action);
                    apply_effects(app, sampler, effects)
                }
                Some(Target::ScrollZone(Tab::Performance)) => {
                    let action = if down {
                        Action::MoveDown
                    } else {
                        Action::MoveUp
                    };
                    let effects = app.dispatch(action);
                    apply_effects(app, sampler, effects)
                }
                Some(Target::ScrollZone(tab)) => {
                    app.scroll_list(tab, if down { 3 } else { -3 });
                    Ok(false)
                }
                Some(Target::Row(_)) => {
                    app.scroll_list(app.tab, if down { 3 } else { -3 });
                    Ok(false)
                }
                _ => Ok(false),
            }
        }
        _ => Ok(false),
    }
}

/// Returns true when the application should exit.
fn apply_effects(app: &mut App, sampler: &mut Sampler, effects: Vec<Effect>) -> Result<bool> {
    let mut quit = false;
    for effect in effects {
        match effect {
            Effect::Quit => quit = true,
            Effect::Refresh => sampler.refresh_now(),
            Effect::SetInterval(interval) => sampler.set_interval(interval),
            Effect::PersistSettings => {
                if let Err(err) = settings::save(&app.settings) {
                    app.notice(err);
                }
            }
            Effect::ClearHistory => {
                sampler.clear_history();
                app.notice("Usage history cleared.");
            }
            Effect::Signal { key, signal, tree } => {
                signal_process(app, key, signal, tree);
            }
            Effect::Service { name, action } => {
                let Some(row) = app
                    .snapshot
                    .services
                    .iter()
                    .find(|row| row.name == name)
                    .cloned()
                else {
                    app.notice("That service is no longer listed.");
                    continue;
                };
                match platform::service_action(&RealRunner, &row, action) {
                    Ok(()) => {
                        app.notice(format!("{} {}.", name, action.past_tense()));
                        sampler.refresh_now();
                    }
                    Err(err) => {
                        app.dialog = Some(Dialog::Error {
                            title: format!("Could not {} {}", action.label().to_lowercase(), name),
                            message: err,
                        });
                    }
                }
            }
            Effect::Startup { name, enabled } => {
                let Some(row) = app
                    .snapshot
                    .startup
                    .iter()
                    .find(|row| row.name == name)
                    .cloned()
                else {
                    app.notice("That startup entry is no longer listed.");
                    continue;
                };
                match platform::set_startup(&RealRunner, &row, enabled) {
                    Ok(()) => {
                        app.notice(format!(
                            "{name} {}.",
                            if enabled { "enabled" } else { "disabled" }
                        ));
                        sampler.refresh_now();
                    }
                    Err(err) => {
                        app.dialog = Some(Dialog::Error {
                            title: format!("Could not change {name}"),
                            message: err,
                        });
                    }
                }
            }
            Effect::Priority { key, nice } => {
                let name = app
                    .snapshot
                    .process(key)
                    .map(|row| row.name.clone())
                    .unwrap_or_else(|| format!("PID {}", key.pid));
                match platform::set_priority(key.pid, nice) {
                    Ok(()) => {
                        app.notice(format!("Priority changed for {name}."));
                        sampler.refresh_now();
                    }
                    Err(err) => {
                        let message = if err.raw_os_error() == Some(libc::EPERM) {
                            "Changing priority for this process needs administrator rights."
                                .to_string()
                        } else {
                            format!("Could not change priority: {err}")
                        };
                        app.dialog = Some(Dialog::Error {
                            title: format!("Could not set priority for {name}"),
                            message,
                        });
                    }
                }
            }
            Effect::Run { program, args } => {
                match run_command(&program, &args) {
                    Ok(message) => app.notice(message),
                    Err(err) => {
                        app.dialog = Some(Dialog::Error {
                            title: format!("Could not run {program}"),
                            message: err,
                        });
                    }
                }
                sampler.refresh_now();
            }
        }
    }
    Ok(quit)
}

fn signal_process(
    app: &mut App,
    key: task_manager_tui::model::ProcKey,
    signal: task_manager_tui::model::Signal,
    tree: bool,
) {
    let Some(row) = app.snapshot.process(key) else {
        app.dialog = Some(Dialog::Error {
            title: format!("Could not send SIG{}", signal.name()),
            message: "The process no longer exists.".to_string(),
        });
        return;
    };
    let name = row.name.clone();
    let mut targets = vec![key.pid];
    if tree {
        let mut descendants =
            task_manager_tui::action::descendants(&app.snapshot.processes, key.pid);
        descendants.reverse();
        targets.extend(descendants.iter().map(|k| k.pid));
    }

    let mut failure = None;
    for pid in targets {
        if let Err(error) = platform::signal(pid, signal) {
            failure = Some((pid, error));
            break;
        }
    }

    match failure {
        None => app.notice(format!("Sent SIG{} to {name}.", signal.name())),
        Some((pid, error)) => {
            let message = match error.raw_os_error() {
                Some(libc::EPERM) => format!(
                    "Operation not permitted for PID {pid}. It belongs to another user or needs administrator rights."
                ),
                Some(libc::ESRCH) => format!("PID {pid} exited before the signal was sent."),
                _ => format!("Signal to PID {pid} failed: {error}"),
            };
            app.dialog = Some(Dialog::Error {
                title: format!("Could not send SIG{}", signal.name()),
                message,
            });
        }
    }
}

fn run_command(program: &str, args: &[String]) -> Result<String, String> {
    let output = std::process::Command::new(program)
        .args(args)
        .output()
        .map_err(|err| format!("{program}: {err}"))?;
    if output.status.success() {
        Ok(format!("{program} finished with success."))
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!(
            "{program} exited with {}: {}",
            output.status,
            stderr.trim()
        ))
    }
}

fn once(app: &mut App) -> Result<()> {
    let mut sampler =
        Sampler::spawn(Some(Duration::from_millis(500))).map_err(|err| anyhow::anyhow!(err))?;
    // The second sample is the first one with CPU deltas, so wait for it.
    let first = sampler.snapshots.recv_timeout(Duration::from_secs(10)).ok();
    match sampler.snapshots.recv_timeout(Duration::from_secs(10)) {
        Ok(snapshot) => app.apply_snapshot(snapshot),
        Err(_) => {
            if let Some(snapshot) = first {
                app.apply_snapshot(snapshot);
            }
        }
    }
    let (width, height) = terminal::size().unwrap_or((120, 40));
    let width = width.max(80);
    let height = height.max(24);
    let mut terminal = Terminal::new(ratatui::backend::TestBackend::new(width, height))?;
    let mut hits = HitMap::default();
    terminal.draw(|frame| ui::draw(frame, app, &mut hits))?;
    let buffer = terminal.backend().buffer();
    let mut stdout = io::stdout();
    for y in 0..height {
        let mut line = String::new();
        for x in 0..width {
            line.push_str(buffer[(x, y)].symbol());
        }
        writeln!(stdout, "{}", line.trim_end())?;
    }
    sampler.stop();
    Ok(())
}

fn set_title(tab: task_manager_tui::app::Tab) {
    let _ = execute!(
        io::stdout(),
        terminal::SetTitle(format!("Task Manager - {}", tab.label()))
    );
}

fn init_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(
        stdout,
        terminal::EnterAlternateScreen,
        cursor::Hide,
        EnableMouseCapture
    )?;
    Terminal::new(CrosstermBackend::new(stdout)).context("cannot create the terminal")
}

/// Restore the terminal exactly once, from any exit path including a panic.
fn restore_terminal() {
    if RESTORED.swap(true, Ordering::SeqCst) {
        return;
    }
    let _ = disable_raw_mode();
    let mut stdout = io::stdout();
    let _ = execute!(
        stdout,
        terminal::LeaveAlternateScreen,
        cursor::Show,
        DisableMouseCapture
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn restore_runs_once() {
        RESTORED.store(false, Ordering::SeqCst);
        restore_terminal();
        restore_terminal();
        assert!(RESTORED.load(Ordering::SeqCst));
    }

    #[test]
    fn run_command_reports_failures() {
        let error = run_command("/bin/sh", &["-c".into(), "exit 3".into()]).unwrap_err();
        assert!(error.contains("exited with"));
    }

    #[test]
    fn run_command_reports_success() {
        let message = run_command("/usr/bin/true", &[]).unwrap();
        assert!(message.contains("success"));
    }
}
