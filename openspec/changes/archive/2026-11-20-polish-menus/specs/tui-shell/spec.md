# Spec Delta

## ADDED Requirements

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

## MODIFIED Requirements

### Requirement: Menu bar is keyboard operable

The menu bar SHALL expose File, Options, View, Tabs, Process, and Help. F10 SHALL focus
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
- **WHEN** the user opens Tabs
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
- **THEN** the Tabs menu opens at its first item

#### Scenario: Narrow menu bar
- **WHEN** the terminal is 40 columns wide
- **THEN** all six menu labels are drawn and clickable, with compact spacing and no truncation

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
