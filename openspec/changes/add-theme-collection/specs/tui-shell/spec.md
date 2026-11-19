# Spec Delta

## MODIFIED Requirements

### Requirement: Light and dark themes

The application SHALL offer six themes - Windows Light, Windows Dark, Nord, Gruvbox Dark,
Phosphor, and Amber - SHALL start in Windows Light unless settings say otherwise, and
MUST keep the chosen theme across restarts.

#### Scenario: Default appearance
- **WHEN** the application starts with no saved settings
- **THEN** Windows Light is used, with a near-white background, dark text, and the Windows 10 blue accent

#### Scenario: Switch theme
- **WHEN** the user selects Dark mode from the Options menu or the Appearance dialog
- **THEN** every visible element switches to the dark palette immediately and the choice is saved

#### Scenario: Readable contrast
- **WHEN** any theme is active
- **THEN** text and its background differ enough to read, and colour is never the only signal for selection

#### Scenario: Cycle themes
- **WHEN** the user presses `t` or picks the next theme in the Appearance dialog
- **THEN** the next theme in the fixed order becomes active and is saved

#### Scenario: Theme set
- **WHEN** the theme picker is opened
- **THEN** it lists Windows Light, Windows Dark, Nord, Gruvbox Dark, Phosphor, and Amber in that order

#### Scenario: Unknown saved theme
- **WHEN** the settings file names a theme that no longer exists
- **THEN** Windows Light is used and a notice explains the fallback

## ADDED Requirements

### Requirement: Chrome and status bands

The menu row and tab strip SHALL share a chrome background distinct from the content
surface, the status bar SHALL use its own background, and the window title MUST name the
application and the active tab.

#### Scenario: Chrome band
- **WHEN** any theme is active
- **THEN** the menu row and tab strip share one background, and the content sits on the lighter surface

#### Scenario: Filter inset
- **WHEN** the filter field is drawn in the menu bar
- **THEN** it keeps a surface-coloured inset so it reads as an input inside the chrome band

#### Scenario: Status bar
- **WHEN** the status bar is drawn
- **THEN** its background differs from both the chrome band and the content surface, and its metrics stay readable

#### Scenario: Window title
- **WHEN** the application starts or the active tab changes
- **THEN** the terminal title reads Task Manager followed by the active tab's name
