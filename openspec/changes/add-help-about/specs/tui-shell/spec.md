# Spec Delta

## ADDED Requirements

### Requirement: Help and About

The help panel SHALL list every global key, and Help > About SHALL open a read-only panel
showing the version, platform, theme, colour depth, settings path, and usage-history path.

#### Scenario: Complete help
- **WHEN** the help panel is opened
- **THEN** it names `i`, the gradient in the F2 row, and both Space actions

#### Scenario: About
- **WHEN** the user selects About from the Help menu
- **THEN** a panel shows the application version, platform, current theme, colour depth, and both paths

#### Scenario: Missing path
- **WHEN** a settings or data directory is unavailable
- **THEN** the About panel shows a dash for that path

#### Scenario: Close
- **WHEN** the user presses Esc, Enter, or q in About
- **THEN** it closes and no other action runs
