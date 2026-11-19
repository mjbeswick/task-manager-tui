# Spec Delta

## ADDED Requirements

### Requirement: Smooth multi-stop gradients

Filled charts and sparklines SHALL blend the chart colour toward the surface through a
three-stop ramp using gamma-correct interpolation, and a plot at least eight rows tall
MUST show a distinct colour on each row of the fill.

#### Scenario: Visible ramp
- **WHEN** a filled chart's plot is eight or more rows tall
- **THEN** the fill uses at least eight distinct colours between the base and the top

#### Scenario: Gamma-correct midpoint
- **WHEN** two colours are blended halfway
- **THEN** the midpoint is computed in linear light rather than by averaging sRGB channels, so the ramp has no muddy centre

#### Scenario: Reduced colour
- **WHEN** the terminal uses the 16-colour palette
- **THEN** the fill stays one flat colour

#### Scenario: Sparkline
- **WHEN** a sparkline is drawn
- **THEN** it uses the same three-stop ramp and the same gamma-correct blend

### Requirement: Value-coloured line cells

Line and Dots charts SHALL colour each braille cell from the highest sample it contains,
so a cell holding a peak is brighter than a cell holding a trough.

#### Scenario: Rising line
- **WHEN** two braille cells hold samples at different heights
- **THEN** the cell holding the higher sample uses the brighter end of the chart ramp

#### Scenario: Shared cell
- **WHEN** one braille cell holds several dots from neighbouring samples
- **THEN** the cell takes the colour of the highest of those samples

#### Scenario: Reduced colour
- **WHEN** the terminal uses the 16-colour palette
- **THEN** line cells keep one chart colour instead of the ramp

### Requirement: Peak-preserving chart resampling

Charts SHALL resample their history to the plot width: linear interpolation when the
plot is wider than the history, and the peak of each covered range when it is narrower.

#### Scenario: Upscale
- **WHEN** 60 samples are drawn into a plot wider than 60 columns
- **THEN** intermediate columns interpolate between neighbouring samples instead of repeating one value

#### Scenario: Downscale
- **WHEN** 60 samples are drawn into a plot narrower than 60 columns
- **THEN** each column holds the peak of the samples it covers, so a one-sample spike is not lost

#### Scenario: Empty history
- **WHEN** fewer than 60 samples exist
- **THEN** the unwritten left side stays empty and is not filled with interpolated zeros
