//! Formula bar widget: name box, function button and the input line, plus the
//! zoom/font-size scrubbers that sit beside it on the ribbon's collapsed bar.

use gridcraft_engine::Tool;

use crate::widgets::scrubby_input::ScrubbyInputWidget;

pub struct FormulaBarWidget {
    pub active_tool: Tool,
    /// Name box contents: the active cell or selected range in A1 form.
    pub name_box: String,
    /// Input line: the text being edited, or the active cell's content.
    pub input_line: String,
    pub font_size: ScrubbyInputWidget,
    pub zoom: ScrubbyInputWidget,
    pub editing: bool,
    /// Multi-line formula bar expanded vs. single row.
    pub expanded: bool,
}

impl FormulaBarWidget {
    pub fn new() -> Self {
        Self {
            active_tool: Tool::Select,
            name_box: "A1".to_string(),
            input_line: String::new(),
            font_size: ScrubbyInputWidget::new("Font Size", 11.0, 1.0, 409.0, "pt"),
            zoom: ScrubbyInputWidget::new("Zoom", 100.0, 10.0, 400.0, "%"),
            editing: false,
            expanded: false,
        }
    }

    pub fn set_tool(&mut self, tool: Tool) {
        self.active_tool = tool;
    }

    pub fn set_name_box(&mut self, a1: impl Into<String>) {
        self.name_box = a1.into();
    }

    /// Begin editing the input line (F2 / double-click / typing).
    pub fn begin_edit(&mut self, initial: &str) {
        self.editing = true;
        self.input_line = initial.to_string();
    }

    /// Commit the input line and return it for `cell.set`.
    pub fn commit(&mut self) -> String {
        self.editing = false;
        std::mem::take(&mut self.input_line)
    }

    /// Escape: drop the in-progress input.
    pub fn cancel(&mut self) {
        self.editing = false;
        self.input_line.clear();
    }

    pub fn toggle_expanded(&mut self) -> bool {
        self.expanded = !self.expanded;
        self.expanded
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_formula_bar_defaults() {
        let mut bar = FormulaBarWidget::new();
        assert_eq!(bar.active_tool, Tool::Select);
        assert_eq!(bar.name_box, "A1");
        assert_eq!(bar.font_size.value, 11.0);
        assert_eq!(bar.zoom.value, 100.0);
        assert!(!bar.editing);

        bar.set_tool(Tool::Fill);
        assert_eq!(bar.active_tool, Tool::Fill);

        bar.begin_edit("=SUM(B2:B10)");
        assert!(bar.editing);
        assert_eq!(bar.commit(), "=SUM(B2:B10)");
        assert!(!bar.editing);

        bar.begin_edit("draft");
        bar.cancel();
        assert!(bar.input_line.is_empty());

        assert!(bar.toggle_expanded());
        assert!(!bar.toggle_expanded());
    }
}
