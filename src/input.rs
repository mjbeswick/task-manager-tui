//! Key events to actions, including the modal states.

use crate::app::{App, Dialog, MENUS, Tab};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Quit,
    NextTab,
    PreviousTab,
    GoTab(usize),
    MoveUp,
    MoveDown,
    PageUp,
    PageDown,
    Top,
    Bottom,
    Expand,
    Collapse,
    SortNext,
    SortPrevious,
    SortFlip,
    ToggleDetails,
    ToggleOptionalColumns,
    ToggleHints,
    ToggleTheme,
    Appearance,
    CycleBack,
    SpeedFaster,
    SpeedSlower,
    Refresh,
    FilterStart,
    FilterChar(char),
    FilterBackspace,
    FilterClear,
    FilterDone,
    EndTask,
    EndTree,
    OpenSignal,
    ToggleSignalTree,
    PriorityChar(char),
    PriorityBackspace,
    PriorityStep(i32),
    ToggleStartup,
    ServiceStart,
    ServiceStop,
    ServiceRestart,
    Inspect,
    SetPriority,
    SetAffinity,
    ShowProperties,
    MenuOpen,
    OpenMenu(usize),
    MenuFirst,
    MenuLast,
    MenuClose,
    MenuLeft,
    MenuRight,
    MenuUp,
    MenuDown,
    MenuActivate,
    Cancel,
    Confirm,
    Help,
    DeleteHistory,
    ClearUsageEntry,
    RunSubmit,
    RunChar(char),
    RunBackspace,
}

pub fn translate(app: &App, key: KeyEvent) -> Option<Action> {
    if key.kind != KeyEventKind::Press {
        return None;
    }
    let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);

    if ctrl && matches!(key.code, KeyCode::Char('c')) {
        return Some(Action::Quit);
    }
    if key.modifiers.contains(KeyModifiers::ALT)
        && let KeyCode::Char(letter) = key.code
    {
        let index = match letter.to_ascii_lowercase() {
            'f' => Some(0),
            'o' => Some(1),
            'v' => Some(2),
            'p' => Some(3),
            'h' => Some(4),
            _ => None,
        };
        if let Some(index) = index {
            return Some(Action::OpenMenu(index));
        }
    }

    if let Some(dialog) = &app.dialog {
        return translate_dialog(dialog, key);
    }

    if let Some(menu) = app.menu {
        return Some(match key.code {
            KeyCode::Esc | KeyCode::F(10) => Action::MenuClose,
            KeyCode::Left => Action::MenuLeft,
            KeyCode::Right => Action::MenuRight,
            KeyCode::Up => Action::MenuUp,
            KeyCode::Down => Action::MenuDown,
            KeyCode::Home => Action::MenuFirst,
            KeyCode::End => Action::MenuLast,
            KeyCode::Enter => Action::MenuActivate,
            _ => {
                let _ = menu;
                return None;
            }
        });
    }

    if app.filter_editing {
        return match key.code {
            KeyCode::Esc => Some(Action::FilterClear),
            KeyCode::Enter => Some(Action::FilterDone),
            KeyCode::Backspace => Some(Action::FilterBackspace),
            KeyCode::Char(ch) => Some(Action::FilterChar(ch)),
            _ => None,
        };
    }

    let action = match key.code {
        KeyCode::Char('q') if !ctrl => Action::Quit,
        KeyCode::F(10) => Action::MenuOpen,
        KeyCode::F(1) => Action::Help,
        KeyCode::F(2) => Action::Appearance,
        KeyCode::F(4) => Action::ToggleHints,
        KeyCode::F(5) => Action::Refresh,
        KeyCode::Tab => Action::NextTab,
        KeyCode::BackTab => Action::PreviousTab,
        KeyCode::Char(ch @ '1'..='7') => Action::GoTab(ch as usize - '1' as usize),
        KeyCode::Up | KeyCode::Char('k') => Action::MoveUp,
        KeyCode::Down | KeyCode::Char('j') => Action::MoveDown,
        KeyCode::Char('d') if ctrl => Action::PageDown,
        KeyCode::Char('u') if ctrl => Action::PageUp,
        KeyCode::Char('g') => Action::Top,
        KeyCode::Char('G') => Action::Bottom,
        KeyCode::PageUp => Action::PageUp,
        KeyCode::PageDown => Action::PageDown,
        KeyCode::Home => Action::Top,
        KeyCode::End => Action::Bottom,
        KeyCode::Enter => match app.tab {
            Tab::Processes => Action::Expand,
            Tab::Details => Action::Inspect,
            Tab::Startup => Action::ShowProperties,
            Tab::Services => Action::ShowProperties,
            _ => Action::Inspect,
        },
        KeyCode::Left | KeyCode::Char('h') => Action::Collapse,
        KeyCode::Right | KeyCode::Char('l') => Action::Expand,
        KeyCode::Delete | KeyCode::Char('e') => Action::EndTask,
        KeyCode::Char('E') => Action::EndTree,
        KeyCode::Char('K') => Action::OpenSignal,
        KeyCode::Char('/') => Action::FilterStart,
        KeyCode::Char('<') | KeyCode::Char(',') => Action::SortPrevious,
        KeyCode::Char('>') | KeyCode::Char('.') => Action::SortNext,
        KeyCode::Char('r') => Action::SortFlip,
        KeyCode::Char('f') => Action::ToggleDetails,
        KeyCode::Char('t') => Action::ToggleTheme,
        KeyCode::Char('+') | KeyCode::Char('=') => Action::SpeedFaster,
        KeyCode::Char('-') => Action::SpeedSlower,
        KeyCode::Char(' ') => match app.tab {
            Tab::Startup => Action::ToggleStartup,
            Tab::Processes => Action::Expand,
            _ => return None,
        },
        KeyCode::Char('?') => Action::Help,
        KeyCode::Char('s') => Action::ServiceStart,
        KeyCode::Char('S') => Action::ServiceStop,
        KeyCode::Char('R') => Action::ServiceRestart,
        KeyCode::Char('i') => Action::Inspect,
        KeyCode::Char('p') => Action::SetPriority,
        KeyCode::Char('x') => Action::ClearUsageEntry,
        KeyCode::Char('a') => Action::SetAffinity,
        KeyCode::Esc => Action::Cancel,
        _ => return None,
    };
    Some(action)
}

fn translate_dialog(dialog: &Dialog, key: KeyEvent) -> Option<Action> {
    match dialog {
        Dialog::Confirm { .. } => match key.code {
            KeyCode::Enter | KeyCode::Char('y') | KeyCode::Char('Y') => Some(Action::Confirm),
            KeyCode::Esc | KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Char('q') => {
                Some(Action::Cancel)
            }
            _ => None,
        },
        Dialog::Error { .. }
        | Dialog::Help
        | Dialog::About
        | Dialog::Properties { .. }
        | Dialog::ProcessInfo { .. } => match key.code {
            KeyCode::Esc | KeyCode::Enter | KeyCode::Char('q') | KeyCode::F(1) => {
                Some(Action::Cancel)
            }
            _ => None,
        },
        Dialog::Appearance { .. } => match key.code {
            KeyCode::Up => Some(Action::MoveUp),
            KeyCode::Down => Some(Action::MoveDown),
            KeyCode::Left => Some(Action::CycleBack),
            KeyCode::Right | KeyCode::Enter => Some(Action::Confirm),
            KeyCode::Esc => Some(Action::Cancel),
            _ => None,
        },
        Dialog::Priority { .. } => match key.code {
            KeyCode::Up | KeyCode::Right => Some(Action::PriorityStep(1)),
            KeyCode::Down | KeyCode::Left => Some(Action::PriorityStep(-1)),
            KeyCode::PageUp => Some(Action::PriorityStep(5)),
            KeyCode::PageDown => Some(Action::PriorityStep(-5)),
            KeyCode::Enter => Some(Action::Confirm),
            KeyCode::Esc => Some(Action::Cancel),
            KeyCode::Backspace => Some(Action::PriorityBackspace),
            KeyCode::Char(ch)
                if ch.is_ascii_digit()
                    || (ch == '-'
                        && !key.modifiers.contains(KeyModifiers::CONTROL)
                        && !key.modifiers.contains(KeyModifiers::ALT)) =>
            {
                Some(Action::PriorityChar(ch))
            }
            _ => None,
        },
        Dialog::Signal { .. } => match key.code {
            KeyCode::Up => Some(Action::MoveUp),
            KeyCode::Down => Some(Action::MoveDown),
            KeyCode::Char(' ') => Some(Action::ToggleSignalTree),
            KeyCode::Enter => Some(Action::Confirm),
            KeyCode::Esc | KeyCode::Char('q') => Some(Action::Cancel),
            _ => None,
        },
        Dialog::Run { .. } => match key.code {
            KeyCode::Esc => Some(Action::Cancel),
            KeyCode::Enter => Some(Action::RunSubmit),
            KeyCode::Backspace => Some(Action::RunBackspace),
            KeyCode::Char(ch)
                if !key.modifiers.contains(KeyModifiers::CONTROL)
                    && !key.modifiers.contains(KeyModifiers::ALT) =>
            {
                Some(Action::RunChar(ch))
            }
            _ => None,
        },
    }
}

pub fn menu_columns() -> usize {
    MENUS.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::{App, Dialog, MenuState};
    use crate::store::settings::Settings;

    fn app() -> App {
        let (app, _) = App::new(Settings::default(), false);
        app
    }

    fn key(code: KeyCode) -> KeyEvent {
        KeyEvent::new(code, KeyModifiers::NONE)
    }

    #[test]
    fn quit_keys() {
        let app = app();
        assert_eq!(
            translate(&app, key(KeyCode::F(4))),
            Some(Action::ToggleHints)
        );
        assert_eq!(translate(&app, key(KeyCode::Char('q'))), Some(Action::Quit));
        assert_eq!(
            translate(
                &app,
                KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL)
            ),
            Some(Action::Quit)
        );
    }

    #[test]
    fn tabs_and_digits() {
        let app = app();
        assert_eq!(translate(&app, key(KeyCode::Tab)), Some(Action::NextTab));
        assert_eq!(
            translate(&app, key(KeyCode::BackTab)),
            Some(Action::PreviousTab)
        );
        assert_eq!(
            translate(&app, key(KeyCode::Char('4'))),
            Some(Action::GoTab(3))
        );
        assert_eq!(
            translate(&app, key(KeyCode::Char('9'))),
            None,
            "only seven tabs"
        );
    }

    #[test]
    fn enter_means_expand_on_processes() {
        let app = app();
        assert_eq!(translate(&app, key(KeyCode::Enter)), Some(Action::Expand));
    }

    #[test]
    fn filter_editing_takes_text() {
        let mut app = app();
        app.filter_editing = true;
        assert_eq!(
            translate(&app, key(KeyCode::Char('a'))),
            Some(Action::FilterChar('a'))
        );
        assert_eq!(
            translate(&app, key(KeyCode::Esc)),
            Some(Action::FilterClear)
        );
        assert_eq!(
            translate(&app, key(KeyCode::Enter)),
            Some(Action::FilterDone)
        );
        assert_eq!(
            translate(&app, key(KeyCode::Char('q'))),
            Some(Action::FilterChar('q')),
            "q types into the filter while editing"
        );
    }

    #[test]
    fn menu_takes_arrows() {
        let mut app = app();
        app.menu = Some(MenuState::default());
        assert_eq!(
            translate(&app, key(KeyCode::Right)),
            Some(Action::MenuRight)
        );
        assert_eq!(translate(&app, key(KeyCode::Up)), Some(Action::MenuUp));
        assert_eq!(
            translate(&app, key(KeyCode::Enter)),
            Some(Action::MenuActivate)
        );
        assert_eq!(translate(&app, key(KeyCode::Esc)), Some(Action::MenuClose));
        assert_eq!(translate(&app, key(KeyCode::Char('q'))), None);
    }

    #[test]
    fn confirm_dialog_answers() {
        let mut app = app();
        app.dialog = Some(Dialog::Confirm {
            title: "End task".into(),
            message: "Sure?".into(),
            confirm_label: "End task".into(),
            action: crate::app::PendingAction::ClearHistory,
        });
        assert_eq!(translate(&app, key(KeyCode::Enter)), Some(Action::Confirm));
        assert_eq!(translate(&app, key(KeyCode::Esc)), Some(Action::Cancel));
        assert_eq!(translate(&app, key(KeyCode::Char('x'))), None);
    }

    #[test]
    fn run_dialog_takes_text() {
        let mut app = app();
        app.dialog = Some(Dialog::Run {
            input: String::new(),
        });
        assert_eq!(
            translate(&app, key(KeyCode::Char('l'))),
            Some(Action::RunChar('l'))
        );
        assert_eq!(
            translate(&app, key(KeyCode::Backspace)),
            Some(Action::RunBackspace)
        );
        assert_eq!(
            translate(&app, key(KeyCode::Enter)),
            Some(Action::RunSubmit)
        );
    }

    #[test]
    fn vim_keys_and_alt_menus() {
        let app = app();
        assert_eq!(
            translate(&app, key(KeyCode::Char('j'))),
            Some(Action::MoveDown)
        );
        assert_eq!(
            translate(&app, key(KeyCode::Char('k'))),
            Some(Action::MoveUp)
        );
        assert_eq!(translate(&app, key(KeyCode::Char('g'))), Some(Action::Top));
        assert_eq!(
            translate(&app, key(KeyCode::Char('G'))),
            Some(Action::Bottom)
        );
        assert_eq!(
            translate(&app, key(KeyCode::Char('l'))),
            Some(Action::Expand)
        );
        assert_eq!(
            translate(&app, key(KeyCode::Char('h'))),
            Some(Action::Collapse)
        );
        assert_eq!(translate(&app, key(KeyCode::Char('?'))), Some(Action::Help));
        assert_eq!(
            translate(&app, key(KeyCode::Char('x'))),
            Some(Action::ClearUsageEntry)
        );
        assert_eq!(
            translate(&app, KeyEvent::new(KeyCode::Char('v'), KeyModifiers::ALT)),
            Some(Action::OpenMenu(2))
        );
        assert_eq!(
            translate(&app, KeyEvent::new(KeyCode::Char('p'), KeyModifiers::ALT)),
            Some(Action::OpenMenu(3))
        );
        assert_eq!(
            translate(&app, KeyEvent::new(KeyCode::Char('h'), KeyModifiers::ALT)),
            Some(Action::OpenMenu(4))
        );
    }

    #[test]
    fn control_keys_page() {
        let app = app();
        assert_eq!(
            translate(
                &app,
                KeyEvent::new(KeyCode::Char('d'), KeyModifiers::CONTROL)
            ),
            Some(Action::PageDown)
        );
        assert_eq!(
            translate(
                &app,
                KeyEvent::new(KeyCode::Char('u'), KeyModifiers::CONTROL)
            ),
            Some(Action::PageUp)
        );
    }

    #[test]
    fn appearance_dialog_keys() {
        let mut app = app();
        app.dialog = Some(Dialog::Appearance { selected: 0 });
        assert_eq!(translate(&app, key(KeyCode::Down)), Some(Action::MoveDown));
        assert_eq!(translate(&app, key(KeyCode::Left)), Some(Action::CycleBack));
        assert_eq!(translate(&app, key(KeyCode::Right)), Some(Action::Confirm));
        assert_eq!(translate(&app, key(KeyCode::Esc)), Some(Action::Cancel));
    }

    #[test]
    fn signal_dialog_keys() {
        let mut app = app();
        assert_eq!(
            translate(&app, key(KeyCode::Char('K'))),
            Some(Action::OpenSignal)
        );
        app.dialog = Some(Dialog::Signal {
            key: crate::model::ProcKey::new(1, 1),
            name: "node".into(),
            pid: 1,
            selected: 0,
            tree: false,
        });
        assert_eq!(translate(&app, key(KeyCode::Up)), Some(Action::MoveUp));
        assert_eq!(translate(&app, key(KeyCode::Down)), Some(Action::MoveDown));
        assert_eq!(
            translate(&app, key(KeyCode::Char(' '))),
            Some(Action::ToggleSignalTree)
        );
        assert_eq!(translate(&app, key(KeyCode::Enter)), Some(Action::Confirm));
        assert_eq!(translate(&app, key(KeyCode::Esc)), Some(Action::Cancel));
    }

    #[test]
    fn priority_dialog_takes_digits_and_steps() {
        let mut app = app();
        app.dialog = Some(Dialog::Priority {
            key: crate::model::ProcKey::new(1, 1),
            name: "node".into(),
            input: "0".into(),
            editing: false,
            error: false,
        });
        assert_eq!(
            translate(&app, key(KeyCode::Char('7'))),
            Some(Action::PriorityChar('7'))
        );
        assert_eq!(
            translate(&app, key(KeyCode::Char('-'))),
            Some(Action::PriorityChar('-'))
        );
        assert_eq!(
            translate(&app, key(KeyCode::Left)),
            Some(Action::PriorityStep(-1))
        );
        assert_eq!(
            translate(&app, key(KeyCode::Right)),
            Some(Action::PriorityStep(1))
        );
        assert_eq!(
            translate(&app, key(KeyCode::PageUp)),
            Some(Action::PriorityStep(5))
        );
        assert_eq!(
            translate(&app, key(KeyCode::PageDown)),
            Some(Action::PriorityStep(-5))
        );
        assert_eq!(
            translate(&app, key(KeyCode::Backspace)),
            Some(Action::PriorityBackspace)
        );
        assert_eq!(translate(&app, key(KeyCode::Enter)), Some(Action::Confirm));
        assert_eq!(translate(&app, key(KeyCode::Char('x'))), None);
    }

    #[test]
    fn process_info_closes_on_enter_and_escape() {
        let mut app = app();
        app.dialog = Some(Dialog::ProcessInfo {
            key: crate::model::ProcKey::new(1, 1),
            name: "node".into(),
        });
        assert_eq!(translate(&app, key(KeyCode::Esc)), Some(Action::Cancel));
        app.dialog = Some(Dialog::About);
        assert_eq!(translate(&app, key(KeyCode::Esc)), Some(Action::Cancel));
        assert_eq!(translate(&app, key(KeyCode::Enter)), Some(Action::Cancel));
        assert_eq!(
            translate(&app, key(KeyCode::Char('q'))),
            Some(Action::Cancel)
        );
    }

    #[test]
    fn release_events_are_ignored() {
        let app = app();
        let release = KeyEvent {
            code: KeyCode::Char('q'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Release,
            state: crossterm::event::KeyEventState::NONE,
        };
        assert_eq!(translate(&app, release), None);
    }
}
