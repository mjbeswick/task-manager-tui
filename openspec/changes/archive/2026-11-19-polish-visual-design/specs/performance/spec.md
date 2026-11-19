## ADDED Requirements

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

## MODIFIED Requirements

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

### Requirement: Chart appearance is adjustable

The user SHALL choose between Filled, Line, Dots, and Blocks rendering and between Per
resource, Teal, Blue, Orange, Magenta, Mono, and Theme colours at View > Appearance or
F2. Per resource MUST be the default with no saved setting. The chosen style and colour
MUST persist across restarts.

#### Scenario: Change the style
- **WHEN** the user selects Line in the Appearance dialog
- **THEN** every chart draws a joined braille line instead of a filled area, and the choice is saved

#### Scenario: Change the colour
- **WHEN** the user selects Orange
- **THEN** chart fills, points, rail sparklines, and per-core graphs use the orange colour immediately

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
