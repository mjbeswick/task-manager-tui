# tui-shell Specification

## Purpose
Presents the Windows 10 Task Manager window - menu bar, seven-tab strip, content
area, and status bar - inside a terminal, driven entirely by the keyboard.

## Requirements

### Requirement: Window layout matches Windows 10

The application SHALL draw a menu bar on its own background at the top, a tab strip
directly beneath it, a rule of centred tab underlines beneath the strip, the active
tab's content in the middle, and a status bar at the bottom, filling the whole
terminal. It MUST use the flat Windows 10 borders rather than a double-line box.

#### Scenario: Standard terminal
- **WHEN** the application starts in a terminal of at least 100 columns by 30 rows
- **THEN** the menu bar, all seven tab labels, the tab rule, the content area, and the status bar are visible without scrolling

#### Scenario: Narrow terminal
- **WHEN** the terminal is narrower than 100 columns
- **THEN** optional columns are dropped first, the process name stays visible, and the tab strip and its rule remain readable

#### Scenario: Terminal below the minimum
- **WHEN** the terminal is smaller than 40 columns by 12 rows
- **THEN** the application shows a single message naming the required minimum size and draws no partial layout

#### Scenario: Menu strip background
- **WHEN** the menu bar is drawn
- **THEN** it fills its row with the chrome background, and the filter field keeps a surface-coloured inset

#### Scenario: Tab rule row
- **WHEN** the tab strip is drawn
- **THEN** one row beneath it is reserved for the tab underlines and is not used for tab labels

### Requirement: Seven tabs in the Windows 10 order

The system SHALL present Processes, Performance, App history, Startup, Users,
Details, and Services, in that order, with labels only. When all seven full labels fit,
the strip SHALL draw them. When they do not fit, the strip MUST be replaced by one
`Tabs: <active> ▾` button naming the active tab. The active tab MUST be drawn as
an open box whose outline uses the rule colour, with an accent label. The button MUST
open a dropdown that lists all seven tabs with their `1`-`7` keys and a mark on the
active tab.

#### Scenario: Cycle tabs
- **WHEN** the user presses Tab or Shift+Tab
- **THEN** the active tab moves to the next or previous tab, wrapping at the ends

#### Scenario: Jump to a tab
- **WHEN** the user presses a digit from 1 to 7
- **THEN** the matching tab becomes active

#### Scenario: Tab state is kept
- **WHEN** the user leaves a tab and returns to it
- **THEN** its selection, scroll position, and sort order are as they were left

#### Scenario: Tab numbers
- **WHEN** the tab strip is drawn
- **THEN** no digits are printed, and the 1-7 keys still jump to their tabs

#### Scenario: Active tab
- **WHEN** a tab is active and the labels are shown
- **THEN** its label uses the accent colour, its outline uses the rule colour, and every other tab keeps the window background

#### Scenario: Centred underlines
- **WHEN** the tab strip draws its labels
- **THEN** each label sits over the continuous rule and the active tab opens through it

#### Scenario: Narrow strip
- **WHEN** the terminal cannot fit all seven full labels
- **THEN** the strip is replaced by one button naming the active tab, with the same raised box and accent label

#### Scenario: Labels only
- **WHEN** the tab strip is drawn
- **THEN** each tab shows its label with no digit prefix

#### Scenario: No separators
- **WHEN** the tab strip is drawn
- **THEN** labels are separated by empty space and no vertical bar appears between them

#### Scenario: Matching outline
- **WHEN** a tab is active and visible
- **THEN** its top border, sides, and outward bottom corners use the same colour as the rule

#### Scenario: Accent label
- **WHEN** a tab is active and visible
- **THEN** only its label uses the accent colour

#### Scenario: Bottom corners
- **WHEN** the rule row is drawn under the active tab
- **THEN** a `┘` closes the left side, a `└` closes the right side, and the space between them is empty

#### Scenario: Active tab hidden
- **WHEN** the strip is in button mode
- **THEN** no separate labels are drawn and the button carries the active tab's name

#### Scenario: Tab button
- **WHEN** the user clicks the button or presses Alt+T
- **THEN** a dropdown lists all seven tabs with their `1`-`7` keys and a mark on the active tab

### Requirement: Menu bar is keyboard operable

The menu bar SHALL expose File, Options, View, Process, and Help. F10 SHALL focus
the menu bar, the arrow keys SHALL move between menus and items, Enter SHALL activate an
item, and Esc SHALL close the menu without acting. On a narrow terminal the labels MUST use
compact spacing so no menu is truncated. Each item MUST show its shortcut hint right-aligned,
groups within a menu MUST be separated by a rule, and the active update speed and each
enabled display toggle MUST carry a mark. An item that cannot act on the current tab MUST be
drawn muted with a short reason and MUST do nothing when activated.

#### Scenario: Open a menu
- **WHEN** the user presses F10 and then Right twice
- **THEN** the View menu opens, its groups are separated by rules, and every item shows its shortcut hint

#### Scenario: Tabs menu
- **WHEN** the user opens the tab list from its button or with Alt+T
- **THEN** it lists Processes, Performance, App history, Startup, Users, Details, and Services with their `1`-`7` keys
- **AND** choosing one switches to that tab

#### Scenario: Process menu
- **WHEN** the user opens Process
- **THEN** it lists End task `Del`, End tree `E`, Send signal `K`, Set priority `p`, and Properties `Enter`

#### Scenario: Options menu
- **WHEN** the user opens Options
- **THEN** it lists the four update speeds with a radio on the active one, the three display toggles with checks, and Appearance

#### Scenario: Alt shortcuts
- **WHEN** the user presses Alt+T
- **THEN** the tab list opens at the active tab

#### Scenario: Narrow menu bar
- **WHEN** the terminal is 40 columns wide
- **THEN** all five menu labels are drawn and clickable, with compact spacing and no truncation

#### Scenario: Activate an item
- **WHEN** an item is highlighted and the user presses Enter
- **THEN** the item's action runs, the menu closes, and focus returns to the content area

#### Scenario: Unavailable item
- **WHEN** an item cannot act on the current tab
- **THEN** it is drawn muted with a short reason in place of its shortcut, and activating it leaves the menu open and changes nothing

#### Scenario: Click the open menu
- **WHEN** the user clicks the label of the menu that is already open
- **THEN** the menu closes

#### Scenario: No process selected
- **WHEN** a process action is chosen on a tab with no selected process
- **THEN** the item is muted with a reason and nothing is signalled

### Requirement: Light and dark themes

The application SHALL offer ten themes - Windows Light, Windows Dark, Nord, Gruvbox Dark,
Phosphor, Amber, Tokyo Night, Catppuccin Mocha, Solarized Light, and Rosé Pine - SHALL
start in Windows Light unless settings say otherwise, and MUST keep the chosen theme
across restarts. Theme switching MUST be available from the `t` key and the Appearance
dialog.

#### Scenario: Default appearance
- **WHEN** the application starts with no saved settings
- **THEN** Windows Light is used, with a near-white background, dark text, and the Windows 10 blue accent

#### Scenario: Switch theme
- **WHEN** the user presses `t` or changes the theme row in the Appearance dialog
- **THEN** every visible element switches to the next palette immediately and the choice is saved

#### Scenario: Readable contrast
- **WHEN** any theme is active
- **THEN** text and its background differ enough to read, and colour is never the only signal for selection

#### Scenario: Cycle themes
- **WHEN** the user presses `t` or picks the next theme in the Appearance dialog
- **THEN** the next theme in the fixed order becomes active and is saved

#### Scenario: Theme set
- **WHEN** the theme picker is opened
- **THEN** it lists Windows Light, Windows Dark, Nord, Gruvbox Dark, Phosphor, Amber, Tokyo Night, Catppuccin Mocha, Solarized Light, and Rosé Pine in that order

#### Scenario: Unknown saved theme
- **WHEN** the settings file names a theme that no longer exists
- **THEN** Windows Light is used and a notice explains the fallback

#### Scenario: No menu duplicate
- **WHEN** the Options menu is opened
- **THEN** it offers update speeds only and no theme entry

### Requirement: Dialog model

The application SHALL present confirmations, errors, the run dialog, and help as
modal panels drawn over the window. A modal panel MUST capture all keys until it
closes, and opening one MUST leave the window behind it unchanged.

#### Scenario: Confirmation
- **WHEN** an action needs confirmation
- **THEN** a panel names the target and the action, offers a default and a cancel choice, and stays open until one is chosen

#### Scenario: Error
- **WHEN** an action fails
- **THEN** a panel shows the failure reason and a hint about privilege, and the underlying table stays unchanged

#### Scenario: Dismiss
- **WHEN** the user presses Esc in any dialog
- **THEN** the dialog closes and no action is taken

#### Scenario: Dimming
- **WHEN** any dialog opens
- **THEN** every colour behind it stays exactly as it was, with no dimming pass

### Requirement: Every action is reachable from the keyboard

Every action the application offers SHALL be reachable from the keyboard, including
arrow and vim-style movement, half-page jumps, first and last row, F2 for Appearance,
and menu access with Alt plus the menu's first letter. No feature MUST require a
pointer.

#### Scenario: No pointer
- **WHEN** the user completes all seven tabs, every menu item, and every dialog using only the keyboard
- **THEN** no control is left unreachable

#### Scenario: Movement styles
- **WHEN** the content area is focused
- **THEN** Up/Down and k/j move one row, PgUp/PgDn and Ctrl+u/Ctrl+d move ten, g and G jump to the first and last row, and h/l collapse and expand

#### Scenario: Direct menu access
- **WHEN** the user presses Alt+F, Alt+O, Alt+V, Alt+P, or Alt+H
- **THEN** the matching menu opens with its first item selected, and Alt+T opens the tab list at the active tab

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

### Requirement: Terminal state restored

The application SHALL restore the terminal on every exit path, including an
unexpected error, and SHALL leave no alternate screen, hidden cursor, or raw mode
behind.

#### Scenario: Normal exit
- **WHEN** the user quits with q or Ctrl+C
- **THEN** the previous screen, cursor, and input mode are restored

#### Scenario: Panic
- **WHEN** the application panics
- **THEN** the terminal is restored before the panic message is printed

### Requirement: Status bar summary

The status bar SHALL show the current CPU, memory, disk, and network totals and the
active update speed, matching the Windows 10 bottom bar, and MUST NOT carry the Fewer
details toggle. Labels MUST use the muted colour while values use the text colour, and
the speed value MUST use the accent.

#### Scenario: Live totals
- **WHEN** a new sample arrives
- **THEN** the status bar totals update with it and keep a fixed width so the layout does not shift

#### Scenario: Unavailable metric
- **WHEN** a metric has no value on this platform
- **THEN** the status bar shows a dash for it and keeps the remaining totals in place

#### Scenario: Details toggle stays elsewhere
- **WHEN** the Processes tab is shown
- **THEN** the Fewer details action appears in the tab's hint line and on the `f` key, not in the status bar

#### Scenario: Label contrast
- **WHEN** the status bar is drawn
- **THEN** each metric label is muted and its value is brighter, without moving the fixed fields

#### Scenario: Speed accent
- **WHEN** the update speed is shown
- **THEN** its value uses the accent colour and the fixed field width is unchanged

### Requirement: Stable status bar layout

The status bar SHALL give each metric a fixed-width field and keep the metric block
anchored at the right edge, so changing values MUST NOT move any other part of the bar.

#### Scenario: Value grows a digit
- **WHEN** a metric crosses from 9.9% to 10.0% or from 999 to 1.0 of a larger unit
- **THEN** the metric block keeps the same width and every other element stays in place

#### Scenario: Position counter
- **WHEN** the selected row changes or the list length gains a digit
- **THEN** the counter keeps a fixed field width and the action label does not move

#### Scenario: Notice
- **WHEN** a notice is shown
- **THEN** it is truncated inside the middle zone and neither the action nor the metrics move

#### Scenario: Narrow terminal
- **WHEN** the terminal is too narrow for every metric
- **THEN** metrics are dropped in a fixed order using their widest possible widths, so the set does not change between samples

### Requirement: Chrome and status bands

The menu row and tab strip SHALL share a chrome background distinct from the content
surface, the status bar SHALL use its own background, and both bands MUST differ visibly
from the window background in every theme. Opening a modal MUST leave those backgrounds
unchanged. The window title MUST name the application and the active tab.

#### Scenario: Chrome band
- **WHEN** any theme is active
- **THEN** the menu row and tab strip share one background, and the content sits on the lighter surface

#### Scenario: Visible bands
- **WHEN** any theme is active
- **THEN** the chrome and status backgrounds each differ from the window background by at least a perceptible channel step

#### Scenario: Modal open
- **WHEN** any dialog is open
- **THEN** the chrome and status backgrounds are the same colours as before it opened

#### Scenario: Filter inset
- **WHEN** the filter field is drawn in the menu bar
- **THEN** it keeps a surface-coloured inset so it reads as an input inside the chrome band

#### Scenario: Status bar
- **WHEN** the status bar is drawn
- **THEN** its background differs from both the chrome band and the content surface, and its metrics stay readable

#### Scenario: Window title
- **WHEN** the application starts or the active tab changes
- **THEN** the terminal title reads Task Manager followed by the active tab's name

### Requirement: Contextual shortcut hints

Each tab SHALL reserve its last content row for one hint line listing that tab's own
shortcuts as `[key] action`, SHALL draw keys in the accent colour and bold against muted
actions, SHALL order the most used actions first, and MUST clip the line at the right edge
instead of wrapping or covering a data row. The user SHALL be able to hide the line, and
the choice MUST persist.

#### Scenario: Processes hints
- **WHEN** the Processes tab is shown
- **THEN** the line reads `[e] End task`, `[E] End tree`, `[K] Signal`, `[p] Priority`, and the current details toggle

#### Scenario: Details hints
- **WHEN** the Details tab is shown
- **THEN** the line names the panel toggle, end task, end tree, signal, and priority

#### Scenario: Services hints
- **WHEN** the Services tab is shown
- **THEN** the line names start, stop, and restart and keeps the scope note

#### Scenario: Startup hints
- **WHEN** a startup entry is selected
- **THEN** the line names enable or disable according to its state and shows the command after the key

#### Scenario: Users hints
- **WHEN** the Users tab is shown
- **THEN** the line names show or hide details according to the panel state

#### Scenario: App history hints
- **WHEN** the App history tab is shown
- **THEN** the line names properties, end app, and clear entry

#### Scenario: Performance hints
- **WHEN** the Performance tab is shown
- **THEN** the line names resource selection and Appearance

#### Scenario: State follows the row
- **WHEN** the selected row or a toggle changes
- **THEN** the hint line updates on the next frame without leaving the previous wording

#### Scenario: Narrow terminal
- **WHEN** the hints are wider than the terminal
- **THEN** the line is cut at the right edge and the table keeps its full width

#### Scenario: No room
- **WHEN** the content area is shorter than four rows
- **THEN** the hint line is not drawn and no row is reserved

#### Scenario: Key emphasis
- **WHEN** the hint line is drawn
- **THEN** every `[key]` including its brackets uses the accent colour and bold, and the action text stays muted

#### Scenario: Hidden
- **WHEN** the user presses F4 or selects Shortcut hints from Options
- **THEN** the line is hidden, no row is reserved, and the choice survives a restart

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
The field MUST show a hint when empty, MUST take focus on `/`, `Ctrl+F`, or a click, and
MUST keep each tab's filter text separate.

#### Scenario: Empty field
- **WHEN** a list tab is active and its filter is empty
- **THEN** the menu bar shows a filter field with a hint naming the `/` key

#### Scenario: Focus
- **WHEN** the user presses `/`, `Ctrl+F`, or clicks the field
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

### Requirement: Help and About

The help panel SHALL list every global key grouped by area, and Help > About SHALL open a
read-only panel showing the version, platform, theme, colour depth, settings path, and
usage-history path. The help panel MUST scroll and MUST keep every key reachable at the
minimum terminal size.

#### Scenario: Complete help
- **WHEN** the help panel is opened
- **THEN** it names `i`, the gradient in the F2 row, and both Space actions

#### Scenario: Short terminal
- **WHEN** the help panel is opened at 40 by 12
- **THEN** Up, Down, PgUp, and PgDn move through every row and the last row can be reached

#### Scenario: About
- **WHEN** the user selects About from the Help menu
- **THEN** a panel shows the application version, platform, current theme, colour depth, and both paths

#### Scenario: Missing path
- **WHEN** a settings or data directory is unavailable
- **THEN** the About panel shows a dash for that path

#### Scenario: Close
- **WHEN** the user presses Esc, Enter, or q in About
- **THEN** it closes and no other action runs

### Requirement: Shortcuts act only in context

The application SHALL apply a tab's single-key actions only while that tab is active,
MUST NOT run a plain-letter command while Ctrl or Alt is held, and MUST NOT open a menu
accelerator while a dialog or the filter field has focus.

#### Scenario: Service keys on another tab
- **WHEN** the user presses `s`, `S`, or `R` while the Processes tab is active
- **THEN** nothing runs, and no service on the Services tab changes

#### Scenario: Ctrl plus a letter
- **WHEN** the user presses `Ctrl+S` or `Ctrl+R`
- **THEN** no service starts, stops, or restarts

#### Scenario: Alt with a dialog
- **WHEN** the confirmation, password, run, or appearance dialog is open and the user presses Alt+F
- **THEN** no menu opens and the dialog stays focused

#### Scenario: Alt while filtering
- **WHEN** the filter field has focus and the user presses Alt+F
- **THEN** no menu opens and no character is inserted

#### Scenario: Filter shortcut
- **WHEN** the user presses `Ctrl+F` on a list tab
- **THEN** the filter field takes focus, as `/` does
