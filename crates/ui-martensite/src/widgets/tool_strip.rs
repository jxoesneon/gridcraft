//! Tool strip widget: single/double column layout, tool flyouts, and the
//! font/fill color chips a spreadsheet toolbar carries.

use gridcraft_engine::Tool;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ToolSlot {
    pub primary: Tool,
    pub alternatives: &'static [Tool],
}

/// Toolbox order mirrors the `Tool` enum: select, edit, fill, format, insert,
/// then the spring-loaded navigation tools.
pub const TOOL_SLOTS: &[ToolSlot] = &[
    ToolSlot { primary: Tool::Select, alternatives: &[] },
    ToolSlot { primary: Tool::EditCell, alternatives: &[] },
    ToolSlot { primary: Tool::Fill, alternatives: &[] },
    ToolSlot { primary: Tool::Format, alternatives: &[] },
    ToolSlot { primary: Tool::Insert, alternatives: &[] },
    ToolSlot { primary: Tool::Hand, alternatives: &[] },
    ToolSlot { primary: Tool::Zoom, alternatives: &[] },
];

pub struct ToolStripWidget {
    pub active_tool: Tool,
    pub double_column: bool,
    /// Font color chip (Excel's underlined-A button).
    pub font_color: [u8; 4],
    /// Fill color chip (the paint-bucket button).
    pub fill_color: [u8; 4],
    /// Format Painter latched (sticky when double-clicked).
    pub format_painter: bool,
}

impl ToolStripWidget {
    pub fn new() -> Self {
        Self {
            active_tool: Tool::Select,
            double_column: false,
            font_color: [0, 0, 0, 255],       // Automatic: black on light sheets
            fill_color: [255, 255, 255, 255], // No Fill: white on dark sheets
            format_painter: false,
        }
    }

    pub fn toggle_column_mode(&mut self) -> bool {
        self.double_column = !self.double_column;
        self.double_column
    }

    pub fn swap_colors(&mut self) {
        std::mem::swap(&mut self.font_color, &mut self.fill_color);
    }

    pub fn reset_default_colors(&mut self) {
        self.font_color = [0, 0, 0, 255];
        self.fill_color = [255, 255, 255, 255];
    }

    pub fn toggle_format_painter(&mut self) -> bool {
        self.format_painter = !self.format_painter;
        self.format_painter
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_strip_state() {
        let mut strip = ToolStripWidget::new();
        assert_eq!(strip.active_tool, Tool::Select);
        assert!(!strip.double_column);
        assert_eq!(strip.font_color, [0, 0, 0, 255]);
        assert_eq!(strip.fill_color, [255, 255, 255, 255]);

        strip.swap_colors();
        assert_eq!(strip.font_color, [255, 255, 255, 255]);
        assert_eq!(strip.fill_color, [0, 0, 0, 255]);

        strip.reset_default_colors();
        assert_eq!(strip.font_color, [0, 0, 0, 255]);

        assert!(strip.toggle_column_mode());
        assert!(strip.double_column);
        assert!(!strip.toggle_column_mode());

        assert!(strip.toggle_format_painter());
        assert!(strip.format_painter);
    }
}
