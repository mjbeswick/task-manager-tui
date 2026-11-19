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

### Requirement: Graphs share one shape

Every graph SHALL draw a filled area rising from the bottom over the last 60 samples,
with a subtle grid, a right-edge value label, and a bottom axis labelled in seconds.

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
