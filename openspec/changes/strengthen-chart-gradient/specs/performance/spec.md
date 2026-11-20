# Spec Delta

## MODIFIED Requirements

### Requirement: Gradient fill

Filled charts SHALL blend the chart colour toward the chart background as the fill rises,
producing a visible vertical gradient at cell resolution, and the top of the fill MUST
stay closer to the chart colour than to the surface.

#### Scenario: Bottom is strongest
- **WHEN** a filled chart is drawn
- **THEN** the rows nearest the axis use the strongest chart colour and the top of the fill is lighter

#### Scenario: Top keeps the hue
- **WHEN** a fill is at least eight rows tall
- **THEN** the row below the tip is closer to the chart colour than to the surface

#### Scenario: Reduced colour
- **WHEN** the terminal uses the 16-colour palette
- **THEN** the fill stays one flat colour rather than approximating the gradient

#### Scenario: Sparkline
- **WHEN** a rail or stat row draws a sparkline
- **THEN** it uses the same gradient as the main filled chart
