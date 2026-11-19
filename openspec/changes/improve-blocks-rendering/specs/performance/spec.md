# Spec Delta

## ADDED Requirements

### Requirement: Blocks edge glyphs

The Blocks style SHALL choose its top-cell glyph so that the fill stays bottom-anchored,
one- and two-eighth steps keep the exact eighth block, no slope is drawn above an empty
older column, a full column keeps a full block, and triangles are used only for steps of
six or more eighths.

#### Scenario: Falling edge
- **WHEN** the older column is higher by four eighths and both sides are filled
- **THEN** the top cell uses `▙`, whose bottom-left is filled, and never a quadrant that
  leaves the bottom-right empty

#### Scenario: Rising edge
- **WHEN** the current column is higher by four eighths and both sides are filled
- **THEN** the top cell uses `▟`

#### Scenario: Small step
- **WHEN** the difference is one or two eighths
- **THEN** the top cell uses the exact eighth block, not a triangle

#### Scenario: Steep step
- **WHEN** the difference is six or more eighths
- **THEN** the top cell uses `◢` for a rise or `◣` for a fall

#### Scenario: Empty neighbour
- **WHEN** the older column has no fill in the row
- **THEN** the top cell uses its eighth block and no slope glyph

#### Scenario: Ceiling
- **WHEN** the column reaches the maximum
- **THEN** the top cell is `█` regardless of the older column

#### Scenario: ASCII
- **WHEN** ASCII glyphs are active
- **THEN** no angular glyph is used and every symbol is ASCII

## MODIFIED Requirements

### Requirement: Peak-preserving chart resampling

Charts SHALL resample their history to the plot width: linear interpolation when the
plot is wider than the history, the exact sample when the widths match, and the peak of
each covered range when it is narrower.

#### Scenario: Upscale
- **WHEN** 60 samples are drawn into a plot wider than 60 columns
- **THEN** intermediate columns interpolate between neighbouring samples instead of repeating one value

#### Scenario: Downscale
- **WHEN** 60 samples are drawn into a plot narrower than 60 columns
- **THEN** each column holds the peak of the samples it covers, so a one-sample spike is not lost

#### Scenario: Native width
- **WHEN** 60 samples are drawn into exactly 60 columns
- **THEN** each column holds its own sample, not the average of two neighbours

#### Scenario: Empty history
- **WHEN** fewer than 60 samples exist
- **THEN** the unwritten left side stays empty and is not filled with interpolated zeros
