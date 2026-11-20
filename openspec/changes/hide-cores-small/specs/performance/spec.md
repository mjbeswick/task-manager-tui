# Spec Delta

## MODIFIED Requirements

### Requirement: Charts are framed

Every chart SHALL be drawn inside a single-line bordered frame whose title names the
series, matching the Windows 10 chart frame. The frame MUST clip the plot instead of
overflowing its panel. Per-processor charts MUST be omitted when their cells are too small
to frame.

#### Scenario: Main chart
- **WHEN** any resource panel is shown
- **THEN** its graph sits inside a border titled with the resource name and its current value is readable on the plot

#### Scenario: Per-core charts
- **WHEN** logical processor charts are shown
- **THEN** each has its own bordered frame titled CPU followed by its index

#### Scenario: Too little room
- **WHEN** the panel cannot give each processor cell at least four rows and ten columns
- **THEN** the per-processor section is hidden and its height goes to the main graph and stat boxes

#### Scenario: Terminal grows
- **WHEN** the terminal becomes large enough for framed cells
- **THEN** the per-processor charts return on the next frame
