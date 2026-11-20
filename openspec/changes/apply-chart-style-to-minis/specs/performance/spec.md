# Spec Delta

## ADDED Requirements

### Requirement: Small charts use the selected style

Rail sparklines and per-core mini charts SHALL render with the same chart style as the
main graph and MUST update when the style changes.

#### Scenario: Filled minis
- **WHEN** Filled is selected
- **THEN** sparklines and mini charts paint filled columns

#### Scenario: Blocks minis
- **WHEN** Blocks is selected
- **THEN** sparklines and mini charts use the Blocks glyph set

#### Scenario: Line and Dots minis
- **WHEN** Line or Dots is selected
- **THEN** sparklines and mini charts use braille points and joins rather than fills

#### Scenario: ASCII minis
- **WHEN** ASCII glyphs are active
- **THEN** mini charts use only ASCII symbols

#### Scenario: Style change
- **WHEN** the user changes the style in Appearance
- **THEN** every visible sparkline and mini chart changes on the next frame
