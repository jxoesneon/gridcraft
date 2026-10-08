//! Fill handle widget: the small square at the selection's corner that drags
//! out a copy/series fill (Excel's AutoFill). Pure geometry here — the engine's
//! `edit.fill*` commands do the actual fill once the drag commits.

use gridcraft_core::{CellRef, RangeRef};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FillDirection {
    Down,
    Right,
    Up,
    Left,
}

pub struct FillHandleWidget {
    /// The source range the handle is anchored to (the selection).
    pub anchor: Option<RangeRef>,
    /// Cell currently under the drag cursor.
    pub drag_to: Option<CellRef>,
}

impl FillHandleWidget {
    pub fn new() -> Self {
        Self { anchor: None, drag_to: None }
    }

    /// Grab the fill handle on `anchor` (the bottom-right corner of the selection).
    pub fn begin(&mut self, anchor: RangeRef) {
        self.anchor = Some(anchor);
        self.drag_to = None;
    }

    /// Extend the drag to `to`. Returns the dominant fill direction, if the
    /// drag leaves the anchor range.
    pub fn update(&mut self, to: CellRef) -> Option<FillDirection> {
        self.drag_to = Some(to);
        self.direction()
    }

    /// The dominant axis of the current drag.
    pub fn direction(&self) -> Option<FillDirection> {
        let anchor = self.anchor?;
        let to = self.drag_to?;
        if to.row > anchor.end.row {
            Some(FillDirection::Down)
        } else if to.row < anchor.start.row {
            Some(FillDirection::Up)
        } else if to.col > anchor.end.col {
            Some(FillDirection::Right)
        } else if to.col < anchor.start.col {
            Some(FillDirection::Left)
        } else {
            None
        }
    }

    /// The range the fill will cover on commit: the anchor extended to the
    /// drag cell along the dominant axis (Excel fills along one axis only).
    pub fn preview(&self) -> Option<RangeRef> {
        let anchor = self.anchor?;
        let to = self.drag_to?;
        match self.direction()? {
            FillDirection::Down => Some(RangeRef::new(anchor.start, CellRef::new(to.row, anchor.end.col))),
            FillDirection::Up => Some(RangeRef::new(CellRef::new(to.row, anchor.start.col), anchor.end)),
            FillDirection::Right => Some(RangeRef::new(anchor.start, CellRef::new(anchor.end.row, to.col))),
            FillDirection::Left => Some(RangeRef::new(CellRef::new(anchor.start.row, to.col), anchor.end)),
        }
    }

    /// Release the handle; returns the committed target range.
    pub fn commit(&mut self) -> Option<RangeRef> {
        let target = self.preview();
        self.anchor = None;
        self.drag_to = None;
        target
    }

    pub fn cancel(&mut self) {
        self.anchor = None;
        self.drag_to = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn c(row: u32, col: u32) -> CellRef {
        CellRef::new(row, col)
    }

    #[test]
    fn test_fill_handle_drag() {
        let mut fill = FillHandleWidget::new();
        fill.begin(RangeRef::new(c(0, 0), c(1, 0))); // A1:A2

        assert_eq!(fill.update(c(5, 0)), Some(FillDirection::Down));
        assert_eq!(fill.preview().map(|r| r.a1()), Some("A1:A6".to_string()));
        assert_eq!(fill.commit().map(|r| r.a1()), Some("A1:A6".to_string()));
        assert!(fill.anchor.is_none());
    }

    #[test]
    fn test_fill_handle_directions() {
        let mut fill = FillHandleWidget::new();
        fill.begin(RangeRef::new(c(4, 4), c(5, 5))); // E5:F6

        assert_eq!(fill.update(c(4, 9)), Some(FillDirection::Right));
        assert_eq!(fill.preview().map(|r| r.a1()), Some("E5:J6".to_string()));

        assert_eq!(fill.update(c(2, 5)), Some(FillDirection::Up));
        assert_eq!(fill.preview().map(|r| r.a1()), Some("E3:F6".to_string()));

        // Inside the anchor: no direction, no preview.
        assert_eq!(fill.update(c(5, 5)), None);
        assert_eq!(fill.preview(), None);

        fill.cancel();
        assert!(fill.drag_to.is_none());
    }
}
