//! Spreadsheet theme tokens.

pub struct Color(pub u8, pub u8, pub u8);

pub struct Theme {
    pub grid_line: Color,
    pub selection_border: Color,
}

impl Theme {
    pub fn sheet_dark() -> Self {
        Self {
            grid_line: Color(45, 50, 60),
            selection_border: Color(16, 185, 129), // Emerald
        }
    }
}
