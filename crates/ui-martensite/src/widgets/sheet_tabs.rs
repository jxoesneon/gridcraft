//! Sheet tab bar widget: the workbook's sheets with activation, rename state,
//! hide/unhide flags and drag reordering.

#[derive(Clone, Debug, PartialEq)]
pub struct SheetTabDef {
    pub id: u64,
    pub name: String,
    pub hidden: bool,
    pub protected: bool,
    pub tab_color: Option<[u8; 4]>,
}

pub struct SheetTabBarWidget {
    pub sheets: Vec<SheetTabDef>,
    pub active_sheet_id: Option<u64>,
    /// Horizontal scroll offset for wide workbooks, in points.
    pub scroll_offset: f32,
}

impl SheetTabBarWidget {
    pub fn new() -> Self {
        Self { sheets: Vec::new(), active_sheet_id: None, scroll_offset: 0.0 }
    }

    pub fn active_index(&self) -> Option<usize> {
        self.sheets.iter().position(|s| Some(s.id) == self.active_sheet_id)
    }

    /// Activate a sheet; hidden sheets cannot be activated.
    pub fn select_sheet(&mut self, id: u64) {
        if self.sheets.iter().any(|s| s.id == id && !s.hidden) {
            self.active_sheet_id = Some(id);
        }
    }

    pub fn toggle_hidden(&mut self, id: u64) {
        if let Some(i) = self.sheets.iter().position(|s| s.id == id) {
            self.sheets[i].hidden = !self.sheets[i].hidden;
            // Hiding the active sheet activates the nearest visible one.
            if self.sheets[i].hidden && self.active_sheet_id == Some(id) {
                self.active_sheet_id = self.sheets.iter().find(|s| !s.hidden).map(|s| s.id);
            }
        }
    }

    /// Move a tab to `to` (clamped), keeping the active sheet tracked by id.
    pub fn reorder(&mut self, id: u64, to: usize) {
        if let Some(from) = self.sheets.iter().position(|s| s.id == id) {
            let tab = self.sheets.remove(from);
            let to = to.min(self.sheets.len());
            self.sheets.insert(to, tab);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sheet(id: u64, name: &str) -> SheetTabDef {
        SheetTabDef { id, name: name.to_string(), hidden: false, protected: false, tab_color: None }
    }

    #[test]
    fn test_sheet_tab_bar() {
        let mut bar = SheetTabBarWidget::new();
        bar.sheets = vec![sheet(1, "Sales"), sheet(2, "Budget"), sheet(3, "Scratch")];
        bar.select_sheet(2);
        assert_eq!(bar.active_index(), Some(1));

        bar.reorder(2, 0);
        assert_eq!(bar.active_index(), Some(0));
        assert_eq!(bar.sheets[0].name, "Budget");

        bar.toggle_hidden(2);
        assert!(bar.sheets[0].hidden);
        // Hiding the active sheet falls back to the first visible sheet.
        assert_eq!(bar.active_sheet_id, Some(1));
        // Hidden sheets reject activation.
        bar.select_sheet(2);
        assert_eq!(bar.active_sheet_id, Some(1));
    }
}
