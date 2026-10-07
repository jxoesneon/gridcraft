//! Spreadsheet menus.

pub struct MenuCategory {
    pub title: &'static str,
    pub items: &'static [&'static str],
}

pub const MENUS: &[MenuCategory] = &[
    MenuCategory { title: "Insert", items: &["sheet.insert_row", "sheet.insert_col"] },
];
