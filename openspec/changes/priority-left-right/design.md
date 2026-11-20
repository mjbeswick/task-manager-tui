# Design

## Context

- `translate_dialog` maps Up and Down to `PriorityStep(1)` and `PriorityStep(-1)`; PgUp and
  PgDn map to plus and minus five.
- Left and Right currently produce `None` in the priority dialog.

## Decisions

### Left is minus, Right is plus

Add `KeyCode::Left => PriorityStep(-1)` and `KeyCode::Right => PriorityStep(1)`. Up stays
plus and Down minus, so both axes work. No new action variant.

*Alternative.* Make Left/Right step by five and keep Up/Down by one. Rejected: the settings
modal uses Left/Right for the small step, and Page keys already cover five.

## Risks / Trade-offs

- [Redundancy] Up/Right and Down/Left do the same thing → Both are conventional; users
  arrive with either habit.
