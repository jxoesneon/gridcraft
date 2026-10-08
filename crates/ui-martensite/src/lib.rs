//! Sovereign retained-mode interface for GridCraft built on the Martensite GUI engine.

pub mod cells;
pub mod command_reg;
pub mod menus;
pub mod shortcuts;
pub mod theme;
pub mod widgets;

use std::sync::{Arc, Mutex};

use gridcraft_core::{CellRef, RangeRef};
use gridcraft_engine::{Engine, Selection};

/// Application state container managing the Martensite GUI pipeline.
pub struct GridcraftApp {
    pub engine: Arc<Mutex<Engine>>,
    pub theme: theme::CraftTheme,
    pub keyboard: shortcuts::KeyboardEngine,
    pub active_tool: gridcraft_engine::Tool,
    pub zoom_level: f32,
    pub pan_offset: [f32; 2],
    /// Text in the formula-bar input line while a cell is being edited.
    pub formula_bar_text: String,
    /// Cell-range state: the active cell plus every selected area.
    pub selection: Selection,
    pub gridlines_visible: bool,
    /// In-cell edit mode (the Enter/Edit mode shown in the status bar).
    pub editing: bool,
    pub is_dirty: bool,
}

impl GridcraftApp {
    pub fn new(engine: Engine) -> Self {
        Self {
            engine: Arc::new(Mutex::new(engine)),
            theme: theme::CraftTheme::dark_neutral(),
            keyboard: shortcuts::KeyboardEngine::new(),
            active_tool: gridcraft_engine::Tool::Select,
            zoom_level: 1.0,
            pan_offset: [0.0, 0.0],
            formula_bar_text: String::new(),
            selection: Selection::default(),
            gridlines_visible: true,
            editing: false,
            is_dirty: false,
        }
    }

    pub fn set_tool(&mut self, tool: gridcraft_engine::Tool) {
        self.active_tool = tool;
    }

    /// Excel-style zoom: 10% to 400%.
    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom_level = zoom.clamp(0.1, 4.0);
    }

    pub fn pan_by(&mut self, dx: f32, dy: f32) {
        self.pan_offset[0] += dx;
        self.pan_offset[1] += dy;
    }

    pub fn reset_view(&mut self) {
        self.zoom_level = 1.0;
        self.pan_offset = [0.0, 0.0];
    }

    pub fn toggle_gridlines(&mut self) -> bool {
        self.gridlines_visible = !self.gridlines_visible;
        self.gridlines_visible
    }

    /// The active (cursor) cell of the selection.
    pub fn active_cell(&self) -> CellRef {
        self.selection.active
    }

    /// Select a single cell, collapsing the selection to it.
    pub fn select_cell(&mut self, cell: CellRef) {
        self.selection = Selection::at(cell);
    }

    /// Select a range; the anchor cell becomes active.
    pub fn select_range(&mut self, range: RangeRef) {
        self.selection = Selection::range(range);
    }

    /// Enter in-cell edit mode (F2 / double-click / typing), seeding the
    /// formula bar with `initial` when given.
    pub fn begin_edit(&mut self, initial: Option<String>) {
        self.editing = true;
        self.formula_bar_text = initial.unwrap_or_default();
    }

    /// Commit the edited text to the active cell and leave edit mode.
    /// Returns the committed input; the caller forwards it to `cell.set`.
    pub fn commit_edit(&mut self) -> String {
        self.editing = false;
        self.is_dirty = true;
        std::mem::take(&mut self.formula_bar_text)
    }

    /// Abandon the in-progress edit (Escape) without marking the sheet dirty.
    pub fn cancel_edit(&mut self) {
        self.editing = false;
        self.formula_bar_text.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gridcraft_engine::Tool;

    #[test]
    fn test_app_initialization() {
        let engine = Engine::new();
        let app = GridcraftApp::new(engine);
        assert_eq!(app.active_tool, Tool::Select);
        assert_eq!(app.zoom_level, 1.0);
        assert_eq!(app.pan_offset, [0.0, 0.0]);
        assert!(app.gridlines_visible);
        assert!(!app.editing);
        assert!(!app.is_dirty);
        assert_eq!(app.active_cell(), CellRef::new(0, 0));
    }

    #[test]
    fn test_zoom_clamping() {
        let engine = Engine::new();
        let mut app = GridcraftApp::new(engine);

        app.set_zoom(2.5);
        assert_eq!(app.zoom_level, 2.5);

        app.set_zoom(0.01);
        assert_eq!(app.zoom_level, 0.1);

        app.set_zoom(1000.0);
        assert_eq!(app.zoom_level, 4.0);
    }

    #[test]
    fn test_pan_and_reset() {
        let engine = Engine::new();
        let mut app = GridcraftApp::new(engine);

        app.pan_by(120.0, -45.0);
        assert_eq!(app.pan_offset, [120.0, -45.0]);

        app.set_zoom(3.0);
        app.reset_view();
        assert_eq!(app.zoom_level, 1.0);
        assert_eq!(app.pan_offset, [0.0, 0.0]);
    }

    #[test]
    fn test_cell_and_range_selection() {
        let engine = Engine::new();
        let mut app = GridcraftApp::new(engine);

        app.select_cell(CellRef::new(5, 2));
        assert_eq!(app.active_cell(), CellRef::new(5, 2));
        assert_eq!(app.selection.current().a1(), "C6");

        app.select_range(RangeRef::new(CellRef::new(0, 0), CellRef::new(9, 3)));
        assert_eq!(app.selection.bounds().a1(), "A1:D10");
    }

    #[test]
    fn test_edit_cycle_and_toggles() {
        let engine = Engine::new();
        let mut app = GridcraftApp::new(engine);

        assert!(app.gridlines_visible);
        assert!(!app.toggle_gridlines());
        assert!(app.toggle_gridlines());

        app.begin_edit(Some("=SUM(".to_string()));
        assert!(app.editing);
        assert_eq!(app.commit_edit(), "=SUM(");
        assert!(app.is_dirty);
        assert!(!app.editing);

        app.begin_edit(None);
        app.cancel_edit();
        assert!(!app.editing);
        assert!(app.formula_bar_text.is_empty());
    }
}
