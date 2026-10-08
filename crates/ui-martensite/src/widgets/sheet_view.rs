//! Sheet view widget with subpixel pan/zoom, gridlines, and marching ants.

use gridcraft_core::CellRef;

/// Default cell metrics in points at 100% zoom (Calibri 11 grid).
pub const DEFAULT_COL_WIDTH: f32 = 64.0;
pub const DEFAULT_ROW_HEIGHT: f32 = 20.0;

pub struct SheetViewWidget {
    pub zoom: f32,
    pub pan_offset: [f32; 2],
    /// Frozen panes: rows/columns pinned at the top-left, in cells.
    pub frozen_rows: u32,
    pub frozen_cols: u32,
    pub gridlines_visible: bool,
    /// Marching-ants animation phase for the copied/selected range outline.
    pub selection_phase: f32,
}

impl SheetViewWidget {
    pub fn new() -> Self {
        Self { zoom: 1.0, pan_offset: [0.0, 0.0], frozen_rows: 0, frozen_cols: 0, gridlines_visible: true, selection_phase: 0.0 }
    }

    /// Excel-style zoom range: 10% to 400%.
    pub fn zoom_at(&mut self, factor: f32, cursor: [f32; 2]) {
        let old_zoom = self.zoom;
        let new_zoom = (self.zoom * factor).clamp(0.1, 4.0);
        let ratio = new_zoom / old_zoom;

        self.pan_offset[0] = cursor[0] - (cursor[0] - self.pan_offset[0]) * ratio;
        self.pan_offset[1] = cursor[1] - (cursor[1] - self.pan_offset[1]) * ratio;
        self.zoom = new_zoom;
    }

    pub fn advance_selection_animation(&mut self, dt: f32) {
        self.selection_phase = (self.selection_phase + dt * 2.0) % 1.0;
    }

    /// Screen point → sheet-space point (points from the grid origin).
    pub fn screen_to_grid(&self, screen: [f32; 2]) -> [f32; 2] {
        [(screen[0] - self.pan_offset[0]) / self.zoom, (screen[1] - self.pan_offset[1]) / self.zoom]
    }

    /// Screen point → cell under the cursor for a uniform grid.
    /// Frozen and variably-sized columns are resolved by the renderer.
    pub fn screen_to_cell(&self, screen: [f32; 2]) -> CellRef {
        let grid = self.screen_to_grid(screen);
        let col = (grid[0] / DEFAULT_COL_WIDTH).max(0.0) as u32;
        let row = (grid[1] / DEFAULT_ROW_HEIGHT).max(0.0) as u32;
        CellRef::new(row, col)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_coordinates_and_zoom() {
        let mut view = SheetViewWidget::new();
        assert_eq!(view.screen_to_grid([100.0, 100.0]), [100.0, 100.0]);
        assert_eq!(view.screen_to_cell([70.0, 45.0]), CellRef::new(2, 1)); // B3

        view.zoom_at(2.0, [0.0, 0.0]);
        assert_eq!(view.zoom, 2.0);
        assert_eq!(view.screen_to_grid([100.0, 100.0]), [50.0, 50.0]);

        view.advance_selection_animation(0.25);
        assert!((view.selection_phase - 0.5).abs() < 1e-4);
    }

    #[test]
    fn test_zoom_clamped_to_excel_range() {
        let mut view = SheetViewWidget::new();
        view.zoom_at(100.0, [0.0, 0.0]);
        assert_eq!(view.zoom, 4.0);
        view.zoom_at(0.0001, [0.0, 0.0]);
        assert_eq!(view.zoom, 0.1);
    }
}
