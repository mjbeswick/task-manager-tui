# table-readability Specification

## Purpose
Keeps wide tables readable by alternating row backgrounds and using a small colour
vocabulary for resource pressure and process or service state.

## Requirements

### Requirement: Alternating row stripes

Every list table SHALL alternate its body-row background between the surface colour and
a subtly different surface colour, starting with the plain surface on the first visible
row.

#### Scenario: Visible stripes
- **WHEN** a table shows more than one body row
- **THEN** adjacent rows differ by a small background step and the columns stay aligned

#### Scenario: Selected row
- **WHEN** a striped row becomes selected
- **THEN** the selection background replaces the stripe for that row

#### Scenario: Headers and rules
- **WHEN** a header, sort rule, hint, or empty-state message is drawn
- **THEN** it keeps the plain surface and is never striped

#### Scenario: Reduced colour
- **WHEN** the terminal uses the 16-colour palette
- **THEN** stripes are omitted rather than approximated by a different named colour

### Requirement: Resource heat escalation

The Processes tab SHALL shade CPU, memory, disk, and network cells by their share of the
system total, SHALL use a warmer colour as the share rises, and MUST reach the danger
colour family in the top step.

#### Scenario: Rising share
- **WHEN** a cell's share crosses a threshold
- **THEN** its background moves one step warmer, from pale to amber to red

#### Scenario: Top step
- **WHEN** a process uses at least 40 percent of the system's CPU or memory
- **THEN** that cell uses the danger background

#### Scenario: Unshaded rows
- **WHEN** a row is selected, or the row is a group heading
- **THEN** its cells are not shaded and keep the row or selection background

#### Scenario: Value stays visible
- **WHEN** a cell is shaded
- **THEN** its numeric value is still printed, so colour is not the only signal

### Requirement: Status colour roles

Tables SHALL colour status words by meaning while keeping the word itself visible.
Not responding MUST use the danger colour; Suspended and Stopped MUST use the warning
colour; Running and Sleeping MUST stay the default text colour. Service Running MUST use
the good colour and service Stopped the muted colour.

#### Scenario: Process warning states
- **WHEN** a process shows Not responding, Suspended, or Stopped
- **THEN** the status word is coloured and the rest of the row is unchanged

#### Scenario: Process default states
- **WHEN** a process shows Running or Sleeping
- **THEN** the status word uses the default text colour

#### Scenario: Service state
- **WHEN** a service shows Running or Stopped
- **THEN** Running is good and Stopped is muted, and the word stays legible

#### Scenario: Selected row
- **WHEN** a row with a coloured status word is selected
- **THEN** the selection style governs the whole row, including that word

### Requirement: Readable at every depth

Stripes, heat steps, and status colours MUST keep text contrast at or above 4.5 to 1 in
both themes and at every supported colour depth.

#### Scenario: Striped text
- **WHEN** text is drawn on a striped row
- **THEN** its contrast against that stripe passes the same threshold as the plain surface

#### Scenario: Heat text
- **WHEN** text is drawn on any heat step, including the danger step
- **THEN** its contrast passes the threshold at truecolor, 256 colours, and 16 colours

#### Scenario: Colour is redundant
- **WHEN** any row is shown
- **THEN** the stripe, heat, and status colour repeat information already carried by position, number, or word
