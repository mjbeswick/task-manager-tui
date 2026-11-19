# Spec Delta

## MODIFIED Requirements

### Requirement: Chart appearance is adjustable

The user SHALL choose between Filled, Line, Dots, and Blocks rendering and between Per
resource, Teal, Blue, Orange, Magenta, Mono, and Theme colours at View > Appearance, F2,
or the `--chart-style` flag. The chosen style and colour MUST persist across restarts.

#### Scenario: Change the style
- **WHEN** the user selects Line in the Appearance dialog
- **THEN** every chart draws a joined braille line instead of a filled area, and the choice is saved

#### Scenario: Change the colour
- **WHEN** the user selects Orange
- **THEN** chart fills, points, and per-core graphs use the orange colour immediately

#### Scenario: Per resource colours
- **WHEN** Per resource is selected
- **THEN** each resource draws with its own colour from the active theme, in both the panel and the rail

#### Scenario: Default colour
- **WHEN** the application starts with no saved chart colour
- **THEN** Per resource is used

#### Scenario: Persist
- **WHEN** the application restarts after an appearance change
- **THEN** the same theme, chart style, and chart colour are used

#### Scenario: Readable on both themes
- **WHEN** the window theme is light or dark
- **THEN** the chosen chart colour stays visible against the chart background

#### Scenario: Follow the theme
- **WHEN** the user selects the Theme chart colour and changes the window theme
- **THEN** charts use the new theme's chart fill immediately

#### Scenario: Blocks style
- **WHEN** the user selects Blocks
- **THEN** filled charts use quadrant templates, eighth blocks, and half-cell triangles to draw the area edge

#### Scenario: Command line style
- **WHEN** the application starts with `--chart-style blocks`
- **THEN** every chart uses the Blocks renderer for that run, whatever the saved style says

#### Scenario: Unknown style
- **WHEN** the flag names a style that does not exist
- **THEN** startup fails and the message lists filled, line, dots, and blocks
