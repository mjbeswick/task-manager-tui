# Spec Delta

## MODIFIED Requirements

### Requirement: Gradient fill

When the gradient option is on, filled charts SHALL blend the chart colour toward the
chart background as the fill rises, producing a visible vertical gradient at cell
resolution, and MUST darken monotonically from base to top with no bright rim on the tip.

#### Scenario: Bottom is strongest
- **WHEN** a filled chart is drawn with gradients on
- **THEN** the rows nearest the axis use the strongest chart colour and the top of the fill is lighter

#### Scenario: Top keeps the hue
- **WHEN** a fill is at least eight rows tall and gradients are on
- **THEN** the top row differs from the base by a visible colour step and is not the surface colour

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
