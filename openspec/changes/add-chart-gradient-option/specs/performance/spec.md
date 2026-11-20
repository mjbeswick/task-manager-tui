# Spec Delta

## ADDED Requirements

### Requirement: Optional chart gradients

The Appearance dialog SHALL offer a Gradient On/Off row, SHALL persist it, and when Off
MUST draw every chart cell in the flat chart colour.

#### Scenario: Turn the gradient off
- **WHEN** the user sets Gradient to Off
- **THEN** filled areas, Blocks slopes, line cells, and sparklines all use one chart colour per series

#### Scenario: Turn the gradient on
- **WHEN** the user sets Gradient to On
- **THEN** the existing ramps return on the next frame

#### Scenario: Persist
- **WHEN** the application restarts
- **THEN** the saved gradient choice is used

#### Scenario: Every chart
- **WHEN** the gradient is Off
- **THEN** the main chart, per-core minis, and rail sparklines are all flat

## MODIFIED Requirements

### Requirement: Gradient fill

When the gradient option is on, filled charts SHALL blend the chart colour toward the
chart background as the fill rises, producing a visible vertical gradient at cell
resolution, and the top of the fill MUST stay closer to the chart colour than to the
surface.

#### Scenario: Bottom is strongest
- **WHEN** a filled chart is drawn with gradients on
- **THEN** the rows nearest the axis use the strongest chart colour and the top of the fill is lighter

#### Scenario: Top keeps the hue
- **WHEN** a fill is at least eight rows tall and gradients are on
- **THEN** the row below the tip is closer to the chart colour than to the surface

#### Scenario: Reduced colour
- **WHEN** the terminal uses the 16-colour palette
- **THEN** the fill stays one flat colour rather than approximating the gradient

#### Scenario: Sparkline
- **WHEN** a rail or stat row draws a sparkline with gradients on
- **THEN** it uses the same gradient as the main filled chart

#### Scenario: Disabled
- **WHEN** the gradient option is off
- **THEN** the fill uses the flat chart colour at every row

### Requirement: Smooth multi-stop gradients

When the gradient option is on, filled charts and sparklines SHALL blend the chart colour
toward the surface through a three-stop ramp using gamma-correct interpolation, and a plot
at least eight rows tall MUST show a distinct colour on each row of the fill.

#### Scenario: Visible ramp
- **WHEN** a filled chart's plot is eight or more rows tall and gradients are on
- **THEN** the fill uses at least eight distinct colours between the base and the top

#### Scenario: Gamma-correct midpoint
- **WHEN** two colours are blended halfway
- **THEN** the midpoint is computed in linear light rather than by averaging sRGB channels, so the ramp has no muddy centre

#### Scenario: Reduced colour
- **WHEN** the terminal uses the 16-colour palette
- **THEN** the fill stays one flat colour

#### Scenario: Sparkline
- **WHEN** a sparkline is drawn with gradients on
- **THEN** it uses the same three-stop ramp and the same gamma-correct blend

#### Scenario: Disabled
- **WHEN** the gradient option is off
- **THEN** no ramp is computed and the chart colour is used directly
