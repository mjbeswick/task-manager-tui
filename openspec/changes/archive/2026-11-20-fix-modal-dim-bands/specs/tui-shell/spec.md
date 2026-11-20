# Spec Delta

## MODIFIED Requirements

### Requirement: Chrome and status bands

The menu row and tab strip SHALL share a chrome background distinct from the content
surface, the status bar SHALL use its own background, and both bands MUST differ visibly
from the window background in every theme. Opening a modal MUST leave those backgrounds
unchanged. The window title MUST name the application and the active tab.

#### Scenario: Chrome band
- **WHEN** any theme is active
- **THEN** the menu row and tab strip share one background, and the content sits on the lighter surface

#### Scenario: Visible bands
- **WHEN** any theme is active
- **THEN** the chrome and status backgrounds each differ from the window background by at least a perceptible channel step

#### Scenario: Modal open
- **WHEN** any dialog is open
- **THEN** the chrome and status backgrounds are the same colours as before it opened

#### Scenario: Filter inset
- **WHEN** the filter field is drawn in the menu bar
- **THEN** it keeps a surface-coloured inset so it reads as an input inside the chrome band

#### Scenario: Status bar
- **WHEN** the status bar is drawn
- **THEN** its background differs from both the chrome band and the content surface, and its metrics stay readable

#### Scenario: Window title
- **WHEN** the application starts or the active tab changes
- **THEN** the terminal title reads Task Manager followed by the active tab's name
