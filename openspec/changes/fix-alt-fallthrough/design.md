# Design

The Alt branch returns `index.map(Action::OpenMenu)` for any Alt+letter, so an unbound
letter returns `None` instead of reaching the plain-key table.

Rejected: listing every dangerous letter. Any Alt+letter is a menu attempt, so none
should act.
