//! Cell coordinate and formula models.

use std::collections::HashMap;

pub struct SheetGrid {
    pub cells: HashMap<(u32, u32), String>,
}

impl SheetGrid {
    pub fn new() -> Self {
        Self { cells: HashMap::new() }
    }

    pub fn set_cell(&mut self, col: u32, row: u32, val: impl Into<String>) {
        self.cells.insert((col, row), val.into());
    }

    pub fn get_cell(&self, col: u32, row: u32) -> Option<&str> {
        self.cells.get(&(col, row)).map(|s| s.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sheet_grid() {
        let mut g = SheetGrid::new();
        g.set_cell(0, 0, "=SUM(A2:A10)");
        assert_eq!(g.get_cell(0, 0), Some("=SUM(A2:A10)"));
    }
}
