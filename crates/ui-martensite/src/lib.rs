//! Sovereign retained-mode spreadsheet UI for GridCraft built on Martensite.

pub mod cells;
pub mod command_reg;
pub mod menus;
pub mod theme;

pub struct GridcraftApp {
    pub sheet: cells::SheetGrid,
    pub formula_bar_text: String,
    pub active_cell: (u32, u32), // (col, row)
}

impl GridcraftApp {
    pub fn new() -> Self {
        Self {
            sheet: cells::SheetGrid::new(),
            formula_bar_text: String::new(),
            active_cell: (0, 0),
        }
    }

    pub fn select_cell(&mut self, col: u32, row: u32) {
        self.active_cell = (col, row);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gridcraft_app() {
        let mut app = GridcraftApp::new();
        assert_eq!(app.active_cell, (0, 0));
        app.select_cell(2, 5);
        assert_eq!(app.active_cell, (2, 5));
    }
}
