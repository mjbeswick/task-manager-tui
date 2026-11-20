# performance Specification

## Purpose
Defines the Performance tab: the resource rail, the 60 second graphs, and the Windows
10 stat boxes for CPU, memory, disk, network, and GPU.

## Requirements

### Requirement: Resource rail and panel

The tab SHALL list CPU, Memory, Disk, Network, and GPU on the left and render the
selected resource's detail panel on the right, using the Windows 10 proportions with
the graph above and stat boxes below. Each rail entry SHALL show a sparkline of its
recent history and its current value.

#### Scenario: Default selection
- **WHEN** the tab opens
- **THEN** CPU is selected and its graph and stat boxes fill the right side

#### Scenario: Move the rail
- **WHEN** the user presses Down on the rail
- **THEN** the next resource is selected and its panel replaces the previous one

#### Scenario: Rail entry content
- **WHEN** the rail has room for three rows per entry
- **THEN** each entry shows its name, a one row sparkline in its chart colour, and a value line such as `27% 3.20 GHz` or `10.0/16.0 GB (62%)`

#### Scenario: Short rail
- **WHEN** the rail has less than three rows per entry
- **THEN** each entry shows its name and value on one row and the sparkline is dropped

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

The user SHALL choose between Filled, Line, Dots, and Blocks rendering and between Per
resource, Teal, Blue, Orange, Magenta, Mono, and Theme colours at View > Appearance, F2,
or the `--chart-style` flag. The chosen style and colour MUST persist across restarts.

#### Scenario: Change the style
- **WHEN** the user selects Line in the Appearance dialog
- **THEN** every chart draws a joined braille line instead of a filled area, and the choice is saved

#### Scenario: Change the colour
- **WHEN** the user selects Orange
- **THEN** chart fills, points, and per-core graphs use the orange colour immediately

#### Scenario: Per resource colours
- **WHEN** Per resource is selected
- **THEN** each resource draws with its own colour from the active theme, in both the panel and the rail

#### Scenario: Default colour
- **WHEN** the application starts with no saved chart colour
- **THEN** Per resource is used

#### Scenario: Persist
- **WHEN** the application restarts after an appearance change
- **THEN** the same theme, chart style, and chart colour are used

#### Scenario: Readable on both themes
- **WHEN** the window theme is light or dark
- **THEN** the chosen chart colour stays visible against the chart background

#### Scenario: Follow the theme
- **WHEN** the user selects the Theme chart colour and changes the window theme
- **THEN** charts use the new theme's chart fill immediately

#### Scenario: Blocks style
- **WHEN** the user selects Blocks
- **THEN** filled charts use quadrant templates, eighth blocks, and half-cell triangles to draw the area edge

#### Scenario: Command line style
- **WHEN** the application starts with `--chart-style blocks`
- **THEN** every chart uses the Blocks renderer for that run, whatever the saved style says

#### Scenario: Unknown style
- **WHEN** the flag names a style that does not exist
- **THEN** startup fails and the message lists filled, line, dots, and blocks

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

### Requirement: Sub-cell line resolution

Line and Dots charts SHALL render through a braille dot matrix of two columns and four
rows per cell, so a sample can land between character rows, and Line MUST join
consecutive samples with a dot-space segment.

#### Scenario: Smooth line
- **WHEN** a sample changes from one height to another
- **THEN** the line between them is continuous at dot resolution rather than stepping by whole rows

#### Scenario: Dots
- **WHEN** the Dots style is active
- **THEN** each sample sets a single dot at its height and no joining segment is drawn

#### Scenario: ASCII mode
- **WHEN** ASCII glyphs are active
- **THEN** Line and Dots fall back to the current marker and vertical bar so every symbol stays ASCII

#### Scenario: Plot extents
- **WHEN** the plot is W columns by H rows
- **THEN** the dot matrix addresses 2W columns and 4H rows without drawing outside the plot

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

### Requirement: Per-resource chart colours come from the theme

The Per resource chart colour SHALL take five distinct colours from the active theme, one
per resource, and MUST update when the theme changes.

#### Scenario: Distinct resources
- **WHEN** Per resource is active
- **THEN** CPU, Memory, Disk, Network, and GPU each draw with a different colour from the palette

#### Scenario: Theme change
- **WHEN** the user switches theme while Per resource is active
- **THEN** every chart adopts the new theme's five resource colours immediately

#### Scenario: Visibility
- **WHEN** any theme is active
- **THEN** each resource colour stays visible against that theme's chart surface

#### Scenario: Other chart colours
- **WHEN** the user chooses Teal, Blue, Orange, Magenta, Mono, or Theme
- **THEN** the fixed colour still overrides the per-resource palette

### Requirement: Filled graphs have a smooth edge

A Filled graph SHALL resolve its top edge to an eighth of a cell, and SHALL draw
that edge in a stronger shade than the fill below it. ASCII mode and 16 colour
mode MUST keep whole and half cell steps in a single shade.

#### Scenario: Partial cell
- **WHEN** a sample's height ends three eighths into a cell
- **THEN** that cell shows a three eighths block, not a full or empty cell

#### Scenario: Edge and fill
- **WHEN** a Filled graph is drawn in Unicode mode at 256 colours or truecolor
- **THEN** the topmost cell of each column uses the edge shade and the cells beneath use the fill shade

#### Scenario: ASCII mode
- **WHEN** the application runs with `--ascii`
- **THEN** the graph uses only ASCII characters

#### Scenario: 16 colour mode
- **WHEN** the application draws at 16 colours
- **THEN** graphs and rail sparklines use only whole and half block steps, all in the chart colour

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
