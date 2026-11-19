# Spec Delta

## MODIFIED Requirements

### Requirement: Chart appearance is adjustable

The user SHALL choose between Filled, Line, and Dots rendering and between Teal, Blue,
Orange, Magenta, Mono, and Theme colours at View > Appearance or F2. The chosen style
and colour MUST persist across restarts.

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

## ADDED Requirements

### Requirement: Sub-cell line resolution

Line and Dots charts SHALL render through a braille dot matrix of two columns and four
rows per cell, so a sample can land between character rows, and Line MUST join
consecutive samples with a dot-space segment.

#### Scenario: Smooth line
- **WHEN** a sample changes from one height to another
- **THEN** the line between them is continuous at dot resolution rather than stepping by whole rows

#### Scenario: Dots
- **WHEN** the Dots style is active
- **THEN** each sample sets a single dot at its height and no joining segment is drawn

#### Scenario: ASCII mode
- **WHEN** ASCII glyphs are active
- **THEN** Line and Dots fall back to the current marker and vertical bar so every symbol stays ASCII

#### Scenario: Plot extents
- **WHEN** the plot is W columns by H rows
- **THEN** the dot matrix addresses 2W columns and 4H rows without drawing outside the plot

### Requirement: Gradient fill

Filled charts SHALL blend the chart colour toward the chart background as the fill rises,
producing a visible vertical gradient at cell resolution.

#### Scenario: Bottom is strongest
- **WHEN** a filled chart is drawn
- **THEN** the rows nearest the axis use the strongest chart colour and the top of the fill is lighter

#### Scenario: Reduced colour
- **WHEN** the terminal uses the 16-colour palette
- **THEN** the fill stays one flat colour rather than approximating the gradient

#### Scenario: Sparkline
- **WHEN** a rail or stat row draws a sparkline
- **THEN** it uses the same gradient as the main filled chart
