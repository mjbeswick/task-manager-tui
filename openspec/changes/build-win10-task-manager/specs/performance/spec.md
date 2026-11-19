# Spec Delta

## Purpose

Defines the Performance tab: the resource rail, the 60 second graphs, and the Windows
10 stat boxes for CPU, memory, disk, network, and GPU.

## ADDED Requirements

### Requirement: Resource rail and panel

The tab SHALL list CPU, Memory, Disk, Network, and GPU on the left and render the
selected resource's detail panel on the right, using the Windows 10 proportions with
the graph above and stat boxes below.

#### Scenario: Default selection
- **WHEN** the tab opens
- **THEN** CPU is selected and its graph and stat boxes fill the right side

#### Scenario: Move the rail
- **WHEN** the user presses Down on the rail
- **THEN** the next resource is selected and its panel replaces the previous one

#### Scenario: Narrow terminal
- **WHEN** the terminal is too narrow for both columns
- **THEN** the rail becomes a single line of labels above the panel and the graph keeps at least twenty columns

### Requirement: Charts are framed

Every chart SHALL be drawn inside a single-line bordered frame whose title names the
series, matching the Windows 10 chart frame. The frame MUST clip the plot instead of
overflowing its panel.

#### Scenario: Main chart
- **WHEN** any resource panel is shown
- **THEN** its graph sits inside a border titled with the resource name and its current value is readable on the plot

#### Scenario: Per-core charts
- **WHEN** logical processor charts are shown
- **THEN** each has its own bordered frame titled CPU followed by its index

#### Scenario: Too little room
- **WHEN** a per-processor cell is shorter than three rows or narrower than eight columns
- **THEN** the frame is dropped for that cell and the graph is drawn without a border rather than clipped badly

### Requirement: Graphs share one shape

Every graph SHALL draw the last 60 samples spread across the full plot width, with a
subtle grid, a right-edge value label, and a bottom axis labelled in seconds. A full
history MUST fill the width at any terminal size.

#### Scenario: Wide terminal
- **WHEN** 60 samples exist in a plot wider than 60 columns
- **THEN** the history covers every column and the left edge is drawn

#### Scenario: Narrow terminal
- **WHEN** the plot is narrower than 60 columns
- **THEN** several samples share a column and the full history still covers the width

### Requirement: Chart appearance is adjustable

The user SHALL choose between Filled, Line, and Dots rendering and between Teal, Blue,
Orange, Magenta, and Mono colours at View > Appearance or F2. The chosen style and
colour MUST persist across restarts.

#### Scenario: Change the style
- **WHEN** the user selects Line in the Appearance dialog
- **THEN** every chart draws points joined by vertical segments instead of a filled area, and the choice is saved

#### Scenario: Change the colour
- **WHEN** the user selects Orange
- **THEN** chart fills, points, and per-core graphs use the orange colour immediately

#### Scenario: Persist
- **WHEN** the application restarts after an appearance change
- **THEN** the same theme, chart style, and chart colour are used

#### Scenario: Readable on both themes
- **WHEN** the window theme is light or dark
- **THEN** the chosen chart colour stays visible against the chart background

### Requirement: One chart label

A resource chart SHALL be labelled once. The Performance rail names the selected
resource and the chart frame MUST NOT repeat it.

#### Scenario: CPU selected
- **WHEN** CPU is the selected resource
- **THEN** the rail reads CPU and the chart frame carries no title

#### Scenario: History
- **WHEN** fewer than 60 samples have been collected
- **THEN** the graph occupies the right portion of its width and leaves the unwritten left part empty rather than drawing zeros

#### Scenario: Full history
- **WHEN** 60 samples exist
- **THEN** the graph spans its full width and scrolls one column per sample

#### Scenario: Value label
- **WHEN** the graph is drawn
- **THEN** its current value is printed at the right edge in the resource's own unit

#### Scenario: Fixed scale
- **WHEN** the resource is a percentage
- **THEN** the vertical scale stays at 0 to 100% so a quiet graph looks quiet

#### Scenario: Adaptive scale
- **WHEN** the resource is a throughput
- **THEN** the vertical scale follows the largest value in the window, with the maximum labelled

### Requirement: CPU panel

The CPU panel SHALL show the labelled stat boxes utilisation, speed, processes,
threads, handles, and uptime, and MUST draw one small graph per logical processor
beneath the main graph.

#### Scenario: Stat boxes
- **WHEN** the CPU panel is visible
- **THEN** each box shows its label, a value, and a unit, and handles shows a dash where the platform has no handle count

#### Scenario: Per-core graphs
- **WHEN** the machine reports more than one logical processor
- **THEN** each has its own graph arranged in a grid that reflows as the terminal width changes

#### Scenario: Single processor
- **WHEN** the machine reports one logical processor
- **THEN** the per-core section is hidden and the main graph takes the space

### Requirement: Memory panel

The memory panel SHALL show in use, available, and total, and SHALL show a dash for
compressed and committed where the platform does not report them.

#### Scenario: Usage
- **WHEN** the memory panel is visible
- **THEN** in use and available are shown with units and their sum matches total within rounding

#### Scenario: Unknown figures
- **WHEN** the platform exposes no compressed or committed figure
- **THEN** those boxes show a dash and a short note, not a zero

### Requirement: Disk panel

The disk panel SHALL show active time, read speed, and write speed for the busiest
disk, and MUST list every physical disk with its own graph and capacity.

#### Scenario: Busiest disk
- **WHEN** several disks are active
- **THEN** the panel headline shows the disk with the highest activity and names it

#### Scenario: Idle disk
- **WHEN** a disk reports no activity
- **THEN** its graph is empty, its speeds read zero, and active time reads zero

### Requirement: Network panel

The network panel SHALL show receive and send throughput and the active adapter name,
and MUST show the link speed as a dash when the platform does not report it.

#### Scenario: Throughput
- **WHEN** traffic flows
- **THEN** receive and send are shown in bits or bytes per second with a unit, and the graph tracks the larger of the two

#### Scenario: No adapter
- **WHEN** the machine has no active network adapter
- **THEN** the panel says so and the rail entry stays selectable

### Requirement: GPU panel

The GPU panel SHALL show per-adapter utilisation and memory when the platform and
driver report them, and MUST show an explicit unavailable message otherwise.

#### Scenario: Reported
- **WHEN** a GPU adapter reports utilisation
- **THEN** its graph and stat boxes show utilisation and memory with the adapter named

#### Scenario: Not reported
- **WHEN** no adapter reports data
- **THEN** the panel shows "Not available on this system" and draws no graph
