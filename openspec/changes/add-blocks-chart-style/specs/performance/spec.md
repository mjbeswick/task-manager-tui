# Spec Delta

## MODIFIED Requirements

### Requirement: Chart appearance is adjustable

The user SHALL choose between Filled, Line, Dots, and Blocks rendering and between Teal,
Blue, Orange, Magenta, Mono, and Theme colours at View > Appearance or F2. The chosen
style and colour MUST persist across restarts.

#### Scenario: Change the style
- **WHEN** the user selects Line in the Appearance dialog
- **THEN** every chart draws a joined braille line instead of a filled area, and the choice is saved

#### Scenario: Change the colour
- **WHEN** the user selects Orange
- **THEN** chart fills, points, and per-core graphs use the orange colour immediately

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
