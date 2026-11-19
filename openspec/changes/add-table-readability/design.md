# Design

## Context

See `proposal.md` - Why.

- `Theme` holds flat RGB roles and two glyph sets. Colour-depth mapping runs once over
  the finished buffer, so the drawing code always works in 24-bit.
- `heat_for` maps a share of the system total to one of five `heat` colours. The
  Processes tab applies it per cell; selected rows skip it.
- Every table builds a per-row base style from `surface_style` or `selected_style`, then
  fills the row and draws into it. `put_match` recolours only the foreground, so a row
  background survives a filter highlight.
- `contrast()` and the existing theme tests already enforce text contrast for text,
  muted text, selection, and every heat step at all three depths.

## Goals / Non-Goals

**Goals:**

- One stripe rule and one tone vocabulary shared by all tables.
- New colours proven by the existing contrast tests, not eyeballed.
- No layout change and no new dependency.

**Non-Goals:**

- Stripes in headers, dialogs, the Performance graphs, or the empty state.
- Configurable stripe strength or a settings toggle.
- New service states; the model still has Running and Stopped only.
- Colour on Users session status or Startup Enabled/Disabled in this change.

## Decisions

### A `surface_alt` role and one row-style helper

Add `surface_alt` to `Theme`: light `#F7F7F7`, dark `#2C2C2C`. A helper
`Theme::row_style(selected, visible_index)` returns selection when selected, otherwise
surface or surface_alt by parity.

Every table calls the helper instead of building the style itself. The parity uses the
visible row index, so the stripe stays aligned to the viewport while rows scroll or
re-sort. Group headings and child rows stripe together, which avoids a broken pattern
when a group expands.

*Alternative.* Stripe by process key. Rejected: the key has no visual meaning, so rows
would appear to change stripe at random as the sort changes.

*Alternative.* Tint every other group. Rejected: with collapsed groups most rows would
share one background, and the pattern would move on every toggle.

### Retuned heat ramp, hotter at the top

`HEAT_STEPS` becomes `[1, 3, 8, 20, 40]` percent. The top two `heat` entries move from
amber toward orange and red, so the ramp ends in the theme's `bad` family instead of a
second amber. 16-colour mode keeps its single step and uses the red entry above 20
percent.

The metric stays share of the whole system, matching the totals row and the displayed
value. On a many-core machine one pinned core sits in the middle of the ramp, not at the
top. A process taking 40 percent of the machine is a real problem on any core count,
which is the honest meaning of the top step.

*Alternative.* Severity by core saturation. Rejected: colour would then disagree with the
number printed beside it.

### Tones instead of colours in the model

`Status` and `ServiceState` gain a `tone()` method returning `Tone::Normal`, `Good`,
`Muted`, `Warn`, or `Danger`. `Theme::tone_style(tone)` maps a tone to a foreground style
on the current row background. The model never names a colour.

Mapping: Not responding -> Danger; Suspended and Stopped -> Warn; Running and Sleeping ->
Normal; service Running -> Good; service Stopped -> Muted.

Tables draw the status word with `tone_style` only when the row is not selected. A
selected row keeps `selected_style` for every cell, because the selection background is a
stronger signal and the tone foreground may not contrast against it.

*Alternative.* Colour the whole row. Rejected: it fights selection, and the word or value
already carries the meaning.

### Numbered tabs and a filled active tab

The strip draws `1 ` through `7 ` before each label. The active tab is filled with
`accent` and `White` text; inactive tabs keep the window background and `muted` text.
Separators stay in the wide layout and disappear in the compact one, because the digits
already segment the strip.

*Alternative.* Keep the underline. Rejected: at a glance the underline is weaker than a
filled block, and the digits double as shortcuts already documented in the key table.
*Alternative.* Add a second tab row. Rejected: the terminal height is the scarce axis.

### Stripes collapse in 16-colour mode

`surface_alt` is mapped by the existing depth pass like any other colour. At 16 colours
it lands on the same named colour as `surface`, so stripes disappear. That is the
intended failure mode: no fake shade and no layout shift.

## Risks / Trade-offs

- [Reorder flicker] CPU sorting can swap two rows between samples, so a stripe pair
  changes → The step is small, and the helper keeps the pattern aligned to position
  rather than identity.
- [Busy cells] Heat plus stripes could look noisy → Heat only paints cells with activity,
  and the stripe is a few percent; most of a row stays one step.
- [Contrast regression] A red heat step may fail on light backgrounds → The existing test
  loop covers every heat entry at every depth; adjust the RGB until it passes rather than
  weakening the test.
- [Selection loses tone] A selected Not-responding row no longer shows red → The status
  word is still on screen, so the meaning survives.
