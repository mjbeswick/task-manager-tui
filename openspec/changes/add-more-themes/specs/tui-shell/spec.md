# Spec Delta

## MODIFIED Requirements

### Requirement: Light and dark themes

The application SHALL offer ten themes - Windows Light, Windows Dark, Nord, Gruvbox Dark,
Phosphor, Amber, Tokyo Night, Catppuccin Mocha, Solarized Light, and Rosé Pine - SHALL
start in Windows Light unless settings say otherwise, and MUST keep the chosen theme
across restarts.

#### Scenario: Default appearance
- **WHEN** the application starts with no saved settings
- **THEN** Windows Light is used, with a near-white background, dark text, and the Windows 10 blue accent

#### Scenario: Switch theme
- **WHEN** the user selects a theme from the Options menu or the Appearance dialog
- **THEN** every visible element switches to that palette immediately and the choice is saved

#### Scenario: Readable contrast
- **WHEN** any theme is active
- **THEN** text and its background differ enough to read, and colour is never the only signal for selection

#### Scenario: Cycle themes
- **WHEN** the user presses `t` or picks the next theme in the Appearance dialog
- **THEN** the next theme in the fixed order becomes active and is saved

#### Scenario: Theme set
- **WHEN** the theme picker is opened
- **THEN** it lists Windows Light, Windows Dark, Nord, Gruvbox Dark, Phosphor, Amber, Tokyo Night, Catppuccin Mocha, Solarized Light, and Rosé Pine in that order

#### Scenario: Unknown saved theme
- **WHEN** the settings file names a theme that no longer exists
- **THEN** Windows Light is used and a notice explains the fallback
