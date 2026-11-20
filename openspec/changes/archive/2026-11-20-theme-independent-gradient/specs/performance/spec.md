# Spec Delta

## MODIFIED Requirements

### Requirement: Gradient fill

When the gradient option is on, filled charts SHALL blend the chart colour toward the
contrasting end of its own luminance, darkening a bright series or lightening a dark one,
so the fade stays visible in every theme. The top of the fill MUST differ clearly from the
base, MUST not be the surface colour, and there MUST be no bright rim on the tip.

#### Scenario: Bottom is strongest
- **WHEN** a filled chart is drawn with gradients on
- **THEN** the rows nearest the axis use the strongest chart colour and the top of the fill is moved toward the contrasting end

#### Scenario: Top keeps the hue
- **WHEN** a fill is at least eight rows tall and gradients are on
- **THEN** the top row differs from the base by a visible colour step and keeps the series hue

#### Scenario: Every theme
- **WHEN** any of the ten themes is active with gradients on
- **THEN** the eight-row fill's top differs from the base by the same minimum step

#### Scenario: No rim
- **WHEN** a filled chart is drawn with gradients on
- **THEN** the tip row is not the raw chart colour and is no brighter than the row below it

#### Scenario: Reduced colour
- **WHEN** the terminal uses the 16-colour palette
- **THEN** the fill stays one flat colour rather than approximating the gradient

#### Scenario: Sparkline
- **WHEN** a rail or stat row draws a sparkline with gradients on
- **THEN** it uses the same gradient as the main filled chart

#### Scenario: Disabled
- **WHEN** the gradient option is off
- **THEN** the fill uses the flat chart colour at every row
