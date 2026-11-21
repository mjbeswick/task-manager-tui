# Design

## Layout

Back to the original chrome:

```
 row 0  menu bar + filter field
 row 1  tab strip: labels, active tab boxed
 row 2  rule, opened under the active tab
 row 3+ active tab's content
 last   status bar
```

## Label tiers

The strip never collapses into a button and never hides a tab.

| Tier | Needs | Labels |
|------|-------|--------|
| Full | 72 columns | Processes, Performance, App history, Startup, Users, Details, Services |
| Short | 56 columns | Proc, Perf, History, Startup, Users, Details, Services |
| Mini | 35 columns | Pro, Per, App, Sta, Usr, Det, Svc |

The tier is chosen from the terminal width, so all seven labels appear at the 40-column
minimum. Each tier keeps one trailing space per label and one empty cell between labels.

## Removed

- `TabMenuState`, `app.tab_menu`, `Action::TabMenuOpen`, `open_tab_menu`,
  `dispatch_tab_menu`, `Target::TabMenuPanel`, `Target::TabMenuItem`, and the `Alt+T`
  binding.
- The `Tabs: <active> ▾` button and its target.
- The help row for `Alt+T`.

## Left to the builder

- Whether `short_label` and `mini_label` live on `Tab` or in `chrome.rs`.
- Whether to keep the tab-dropdown tests as strip tests or delete them.
