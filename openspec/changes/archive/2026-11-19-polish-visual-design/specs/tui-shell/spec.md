## ADDED Requirements

### Requirement: Colour depth adapts to the terminal

The application SHALL detect whether the terminal renders truecolor, 256 colours,
or 16 colours, and SHALL draw with a palette mapped to that depth. A `--color`
flag with `auto`, `truecolor`, `256`, and `16` MUST override detection.

#### Scenario: Truecolor terminal
- **WHEN** `COLORTERM` is `truecolor` or `24bit`
- **THEN** every colour is drawn as the exact theme value

#### Scenario: 256 colour terminal
- **WHEN** `COLORTERM` is unset and `TERM` names a 256 colour terminal
- **THEN** every theme colour is drawn as its nearest xterm 256 palette entry

#### Scenario: Linux virtual console
- **WHEN** `TERM` is `linux`
- **THEN** the application uses the 16 colour palette, selection and focus stay distinguishable, and heat shading uses two steps

#### Scenario: Override
- **WHEN** the user starts with `--color 256` in a truecolor terminal
- **THEN** the 256 colour mapping is used for that run

#### Scenario: Readable at every depth
- **WHEN** any theme is drawn at any colour depth
- **THEN** body text keeps at least 4.5:1 contrast against its background

### Requirement: Filter field on list tabs

Every tab that shows a list SHALL show a filter field at the right of the menu bar.
The field MUST show a hint when empty, MUST take focus on `/` or a click, and MUST
keep each tab's filter text separate.

#### Scenario: Empty field
- **WHEN** a list tab is active and its filter is empty
- **THEN** the menu bar shows a filter field with a hint naming the `/` key

#### Scenario: Focus
- **WHEN** the user presses `/` or clicks the field
- **THEN** the field shows a cursor and typed characters go into it

#### Scenario: Leave the field
- **WHEN** the user presses Enter in the field
- **THEN** focus returns to the list and the filter stays applied

#### Scenario: Clear
- **WHEN** the user presses Esc in the field or clicks its clear mark
- **THEN** the filter empties and every row returns

#### Scenario: Matches are highlighted
- **WHEN** a filter is applied
- **THEN** the matching part of each visible name is drawn in the accent style

#### Scenario: Tab without a list
- **WHEN** the Performance tab is active
- **THEN** no filter field is drawn

#### Scenario: Narrow terminal
- **WHEN** the menu bar has no room for both menus and the field
- **THEN** the field shrinks to its icon and hint key, and the menus stay whole

### Requirement: Column headers sort on click

Every sortable table SHALL sort by a column when its header is clicked. Clicking
the column that is already sorted MUST reverse its direction.

#### Scenario: Click a new column
- **WHEN** the user clicks the Memory header while CPU is sorted
- **THEN** rows reorder by memory, highest first, and the header shows the direction marker

#### Scenario: Click the sorted column
- **WHEN** the user clicks the header of the sorted column
- **THEN** the order reverses and the marker flips

#### Scenario: Keyboard equivalent
- **WHEN** the user presses `<`, `>`, or `r`
- **THEN** the same sort changes happen without the mouse

## MODIFIED Requirements

### Requirement: Mouse support

The application SHALL capture mouse input and SHALL treat clicks as shortcuts for
actions that also exist on the keyboard. A click MUST NOT be the only way to do
anything, and the terminal's mouse mode MUST be released on exit.

#### Scenario: Click a tab
- **WHEN** the user clicks a tab label
- **THEN** that tab becomes active and its previous state is restored

#### Scenario: Click a row
- **WHEN** the user clicks a table row
- **THEN** that row becomes selected, and a second click on the same row within 400 milliseconds performs the row's Enter action

#### Scenario: Click the menus
- **WHEN** the user clicks a menu label and then an item
- **THEN** the menu opens on that label and the item's action runs

#### Scenario: Click away closes a menu
- **WHEN** a menu is open and the user clicks outside its label and dropdown
- **THEN** the menu closes and the click does not select a row, switch a tab, sort a column, or start a filter

#### Scenario: Click inside the dropdown
- **WHEN** a menu is open and the user clicks its border or empty space
- **THEN** the menu stays open and nothing underneath is activated

#### Scenario: Click a resource
- **WHEN** the user clicks CPU, Memory, Disk, Network, or GPU in the Performance rail
- **THEN** that resource's panel is shown

#### Scenario: Click a column header
- **WHEN** the user clicks a sortable column header
- **THEN** the table sorts as the Column headers sort on click requirement describes

#### Scenario: Click the filter field
- **WHEN** the user clicks the filter field
- **THEN** the field takes focus as if `/` were pressed

#### Scenario: Wheel
- **WHEN** the user scrolls the wheel over the content area
- **THEN** the viewport under the pointer moves three rows in the wheel's direction and the selection does not change

#### Scenario: Click dialog buttons
- **WHEN** the user clicks a confirm, cancel, or close button in a dialog
- **THEN** the same action runs as its key would run

#### Scenario: Mouse mode restored
- **WHEN** the application exits, including through a panic
- **THEN** mouse reporting is disabled along with the alternate screen and raw mode
