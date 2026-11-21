//! Interactive regions, recorded while drawing and looked up by the mouse.
//!
//! Drawing is the only code that knows where a tab or a row ended up, so the
//! frame records its own hit areas instead of a second layout copy drifting.

use crate::app::{ResourceChoice, Tab};
use crate::sample::group::SortKey;
use ratatui::layout::Rect;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Target {
    /// A menu bar label, by menu index.
    Menu(usize),
    /// A menu dropdown item, by menu and item index.
    MenuItem(usize, usize),
    /// The dropdown panel itself, including its border and empty space.
    MenuPanel,
    Tab(Tab),
    /// A table row, by index in the tab's visible row list.
    Row(usize),
    Resource(ResourceChoice),
    DialogConfirm,
    DialogCancel,
    DialogSignal(usize),
    AppearanceRow(usize),
    DialogClose,
    /// A sortable column header.
    Header(SortKey),
    /// The filter field in the menu bar, and its clear mark.
    FilterField,
    FilterClear,
    /// The rows of a list, so the wheel knows what it is over.
    ScrollZone(Tab),
    /// The scrollbar track and its draggable thumb.
    ScrollbarTrack(Tab),
    ScrollbarThumb(Tab),
}

#[derive(Debug, Default)]
pub struct HitMap {
    entries: Vec<(Rect, Target)>,
}

impl HitMap {
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    pub fn add(&mut self, rect: Rect, target: Target) {
        if rect.width == 0 || rect.height == 0 {
            return;
        }
        self.entries.push((rect, target));
    }

    /// Record one header target per visible column, laid out left to right
    /// from `x`. A `None` width is a column the layout dropped.
    pub fn header(&mut self, x: u16, y: u16, height: u16, columns: &[(Option<u16>, SortKey)]) {
        let mut x = x;
        for (width, key) in columns {
            if let Some(width) = width {
                self.add(Rect::new(x, y, *width, height), Target::Header(*key));
                x += width;
            }
        }
    }

    pub fn hit(&self, column: u16, row: u16) -> Option<Target> {
        self.entries
            .iter()
            .rev()
            .find(|(rect, _)| {
                column >= rect.x
                    && column < rect.x + rect.width
                    && row >= rect.y
                    && row < rect.y + rect.height
            })
            .map(|(_, target)| *target)
    }

    /// The rectangle recorded for a target, when it has one.
    pub fn rect_of(&self, target: Target) -> Option<Rect> {
        self.entries
            .iter()
            .rev()
            .find(|(_, recorded)| *recorded == target)
            .map(|(rect, _)| *rect)
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hit_finds_the_topmost_region() {
        let mut hits = HitMap::default();
        hits.add(Rect::new(0, 0, 10, 2), Target::Tab(Tab::Processes));
        hits.add(Rect::new(0, 0, 10, 1), Target::Tab(Tab::Processes));
        assert_eq!(hits.hit(3, 0), Some(Target::Tab(Tab::Processes)));
        assert_eq!(hits.hit(3, 1), Some(Target::Tab(Tab::Processes)));
        assert_eq!(hits.hit(20, 0), None);
    }

    #[test]
    fn empty_regions_are_ignored() {
        let mut hits = HitMap::default();
        hits.add(Rect::new(0, 0, 0, 5), Target::Tab(Tab::Processes));
        assert!(hits.is_empty());
        hits.add(Rect::new(0, 0, 4, 1), Target::Row(2));
        assert_eq!(hits.len(), 1);
        assert_eq!(hits.hit(1, 0), Some(Target::Row(2)));
    }
}
