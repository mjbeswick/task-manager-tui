# Spec Delta

## Purpose

Defines how pointer scrolling works in list tabs: which region receives a wheel event, the
scrollbar and its drag and page gestures, the viewport model that keeps pointer scrolling
separate from the selection, and the fallback for narrow terminals.

## ADDED Requirements

### Requirement: The wheel acts on the region under the pointer

A wheel event SHALL be applied to the region the pointer is over: a table scrolls that
table, the Performance rail moves the selected resource, an open menu moves its
highlight, and any other area ignores the event.

#### Scenario: Wheel over a table
- **WHEN** the pointer is over the rows and the wheel turns down
- **THEN** that list's viewport moves three rows down and no other region changes

#### Scenario: Wheel over the rail
- **WHEN** the pointer is over the Performance rail and the wheel turns
- **THEN** the selected resource moves to the next or previous entry and the table is untouched

#### Scenario: Wheel over an open menu
- **WHEN** a menu is open and the pointer is over its items
- **THEN** the highlighted item moves one step and the menu stays open

#### Scenario: Wheel elsewhere
- **WHEN** the pointer is over the menu bar, status bar, or empty space
- **THEN** nothing scrolls

#### Scenario: Trackpad bursts
- **WHEN** one gesture delivers several wheel events in a row
- **THEN** each event moves three rows and no event is dropped

### Requirement: A scrollbar shows when a list overflows

The application SHALL draw a one-column scrollbar at the right edge of a list whose rows
exceed its viewport, with a thumb whose size and position reflect the visible fraction
and the viewport offset. A list that fits MUST NOT show one.

#### Scenario: Long list
- **WHEN** 300 rows exist and 20 are visible
- **THEN** the scrollbar spans the full track height with a thumb about one fifteenth as tall, at the position matching the viewport

#### Scenario: Short list
- **WHEN** every row fits in the viewport
- **THEN** no scrollbar is drawn and the table keeps its full width

#### Scenario: Both themes
- **WHEN** the light or dark theme is active
- **THEN** the track and thumb differ from the table background enough to see

#### Scenario: Column cost
- **WHEN** the scrollbar is visible
- **THEN** the table's columns lose exactly one column and no value is clipped by the bar

### Requirement: The thumb can be dragged

Pressing the thumb and moving the pointer SHALL move the viewport in proportion to the
pointer's travel, and releasing SHALL stop the drag. Dragging to either end MUST reach
the first or last row.

#### Scenario: Drag down
- **WHEN** the user drags the thumb from the top of the track to the middle
- **THEN** the viewport moves about half way through the list and keeps the same number of visible rows

#### Scenario: Drag to the end
- **WHEN** the user drags the thumb past the bottom of the track
- **THEN** the viewport stops with the last row visible at the bottom

#### Scenario: Release
- **WHEN** the pointer button is released anywhere
- **THEN** the drag ends and later pointer movement does not scroll

#### Scenario: Selection during a drag
- **WHEN** the user drags the thumb
- **THEN** the selected row does not change

### Requirement: Clicking the track pages

A click above or below the thumb SHALL move the viewport by one viewport's worth of rows
less one, so context is kept at the edge, and MUST clamp at the ends.

#### Scenario: Page down
- **WHEN** the user clicks the track below the thumb with 20 visible rows
- **THEN** the viewport moves 19 rows down

#### Scenario: Page up
- **WHEN** the user clicks the track above the thumb
- **THEN** the viewport moves 19 rows up

#### Scenario: At the end
- **WHEN** the viewport is already at the last page and the track is clicked below
- **THEN** nothing moves

### Requirement: The viewport is separate from the selection

Each list SHALL keep a viewport offset independent of its selected row. Keyboard
movement MUST keep the selection visible. Pointer scrolling MUST NOT change the
selection.

#### Scenario: Pointer scroll
- **WHEN** the wheel or scrollbar moves the viewport
- **THEN** the selected row stays selected and may leave the view

#### Scenario: Keyboard returns
- **WHEN** the selection is outside the view and the user presses an arrow key
- **THEN** the selection becomes visible at the nearest edge and then moves normally

#### Scenario: Sample clamp
- **WHEN** a sample removes rows and the viewport would pass the new end
- **THEN** the viewport is clamped so the last page still shows rows

#### Scenario: Filter reset
- **WHEN** the filter changes the row count
- **THEN** the viewport is clamped and the selected row stays visible if it survived the filter

### Requirement: Narrow terminals keep the counter

A terminal narrower than 60 columns SHALL draw no scrollbar and MUST keep the position
counter in the status bar, so the position is never lost.

#### Scenario: Narrow list
- **WHEN** the terminal is 50 columns wide
- **THEN** no scrollbar column is reserved and the status bar still reads `12/300`

#### Scenario: Wide list
- **WHEN** the terminal is 120 columns wide
- **THEN** both the scrollbar and the counter are present and agree on the position
