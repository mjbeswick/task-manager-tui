# Spec Delta

## MODIFIED Requirements

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
