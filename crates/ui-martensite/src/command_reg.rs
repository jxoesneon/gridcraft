//! Decoupled command catalog and taxonomy for GridCraft.
//!
//! Every `id` is a real engine command (see `gridcraft_engine::find_command`);
//! ids follow Excel's ribbon naming (`home.bold`, `insert.chart`, `data.sortAscending`).

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CommandCategory {
    File,
    Edit,
    View,
    Insert,
    Format,
    Formulas,
    Data,
    Window,
    Help,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CommandSpec {
    pub id: &'static str,
    pub label: &'static str,
    pub category: CommandCategory,
    pub default_shortcut: Option<&'static str>,
    pub secondary_shortcut: Option<&'static str>,
}

pub const COMMAND_REGISTRY: &[CommandSpec] = &[
    // File
    CommandSpec { id: "file.new", label: "New Workbook", category: CommandCategory::File, default_shortcut: Some("Cmd+N"), secondary_shortcut: None },
    CommandSpec { id: "file.open", label: "Open…", category: CommandCategory::File, default_shortcut: Some("Cmd+O"), secondary_shortcut: None },
    CommandSpec { id: "file.save", label: "Save", category: CommandCategory::File, default_shortcut: Some("Cmd+S"), secondary_shortcut: Some("F12") },
    CommandSpec {
        id: "file.saveAs",
        label: "Save As…",
        category: CommandCategory::File,
        default_shortcut: Some("Cmd+Shift+S"),
        secondary_shortcut: None,
    },
    CommandSpec { id: "file.print", label: "Print…", category: CommandCategory::File, default_shortcut: Some("Cmd+P"), secondary_shortcut: None },
    CommandSpec { id: "file.exportPdf", label: "Export as PDF", category: CommandCategory::File, default_shortcut: None, secondary_shortcut: None },
    CommandSpec { id: "file.exportCsv", label: "Export as CSV", category: CommandCategory::File, default_shortcut: None, secondary_shortcut: None },
    CommandSpec { id: "file.close", label: "Close", category: CommandCategory::File, default_shortcut: Some("Cmd+W"), secondary_shortcut: None },
    // Edit
    CommandSpec { id: "edit.undo", label: "Undo", category: CommandCategory::Edit, default_shortcut: Some("Cmd+Z"), secondary_shortcut: None },
    CommandSpec { id: "edit.redo", label: "Redo", category: CommandCategory::Edit, default_shortcut: Some("Cmd+Y"), secondary_shortcut: Some("F4") },
    CommandSpec {
        id: "edit.cut",
        label: "Cut",
        category: CommandCategory::Edit,
        default_shortcut: Some("Cmd+X"),
        secondary_shortcut: Some("Shift+Delete"),
    },
    CommandSpec {
        id: "edit.copy",
        label: "Copy",
        category: CommandCategory::Edit,
        default_shortcut: Some("Cmd+C"),
        secondary_shortcut: Some("Ctrl+Insert"),
    },
    CommandSpec {
        id: "edit.paste",
        label: "Paste",
        category: CommandCategory::Edit,
        default_shortcut: Some("Cmd+V"),
        secondary_shortcut: Some("Shift+Insert"),
    },
    CommandSpec {
        id: "edit.pasteSpecial",
        label: "Paste Special…",
        category: CommandCategory::Edit,
        default_shortcut: Some("Ctrl+Alt+V"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "edit.formatPainter",
        label: "Format Painter",
        category: CommandCategory::Edit,
        default_shortcut: None,
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "edit.fillDown",
        label: "Fill Down",
        category: CommandCategory::Edit,
        default_shortcut: Some("Cmd+D"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "edit.fillRight",
        label: "Fill Right",
        category: CommandCategory::Edit,
        default_shortcut: Some("Cmd+R"),
        secondary_shortcut: None,
    },
    CommandSpec { id: "edit.fillSeries", label: "Fill Series…", category: CommandCategory::Edit, default_shortcut: None, secondary_shortcut: None },
    CommandSpec {
        id: "edit.selectAll",
        label: "Select All",
        category: CommandCategory::Edit,
        default_shortcut: Some("Cmd+A"),
        secondary_shortcut: None,
    },
    CommandSpec { id: "edit.find", label: "Find…", category: CommandCategory::Edit, default_shortcut: Some("Cmd+F"), secondary_shortcut: None },
    CommandSpec {
        id: "edit.goTo",
        label: "Go To…",
        category: CommandCategory::Edit,
        default_shortcut: Some("Ctrl+G"),
        secondary_shortcut: Some("F5"),
    },
    // View
    CommandSpec { id: "view.normal", label: "Normal", category: CommandCategory::View, default_shortcut: None, secondary_shortcut: None },
    CommandSpec {
        id: "view.pageBreakPreview",
        label: "Page Break Preview",
        category: CommandCategory::View,
        default_shortcut: None,
        secondary_shortcut: None,
    },
    CommandSpec { id: "view.gridlines", label: "Gridlines", category: CommandCategory::View, default_shortcut: None, secondary_shortcut: None },
    CommandSpec { id: "view.headings", label: "Headings", category: CommandCategory::View, default_shortcut: None, secondary_shortcut: None },
    CommandSpec { id: "view.zoom", label: "Zoom…", category: CommandCategory::View, default_shortcut: None, secondary_shortcut: None },
    CommandSpec {
        id: "view.zoomToSelection",
        label: "Zoom to Selection",
        category: CommandCategory::View,
        default_shortcut: None,
        secondary_shortcut: None,
    },
    CommandSpec { id: "view.freezePanes", label: "Freeze Panes", category: CommandCategory::View, default_shortcut: None, secondary_shortcut: None },
    CommandSpec {
        id: "view.freezeTopRow",
        label: "Freeze Top Row",
        category: CommandCategory::View,
        default_shortcut: None,
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "view.freezeFirstColumn",
        label: "Freeze First Column",
        category: CommandCategory::View,
        default_shortcut: None,
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "view.unfreezePanes",
        label: "Unfreeze Panes",
        category: CommandCategory::View,
        default_shortcut: None,
        secondary_shortcut: None,
    },
    CommandSpec { id: "view.split", label: "Split", category: CommandCategory::View, default_shortcut: None, secondary_shortcut: None },
    // Insert
    CommandSpec {
        id: "home.insertCells",
        label: "Insert Cells…",
        category: CommandCategory::Insert,
        default_shortcut: None,
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "home.insertRows",
        label: "Insert Sheet Rows",
        category: CommandCategory::Insert,
        default_shortcut: None,
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "home.insertColumns",
        label: "Insert Sheet Columns",
        category: CommandCategory::Insert,
        default_shortcut: None,
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "home.insertSheet",
        label: "Insert Sheet",
        category: CommandCategory::Insert,
        default_shortcut: Some("Shift+F11"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "insert.chart",
        label: "Chart…",
        category: CommandCategory::Insert,
        default_shortcut: Some("Alt+F1"),
        secondary_shortcut: Some("F11"),
    },
    CommandSpec { id: "insert.table", label: "Table", category: CommandCategory::Insert, default_shortcut: Some("Cmd+T"), secondary_shortcut: None },
    CommandSpec { id: "insert.pivotTable", label: "PivotTable", category: CommandCategory::Insert, default_shortcut: None, secondary_shortcut: None },
    CommandSpec { id: "insert.picture", label: "Picture…", category: CommandCategory::Insert, default_shortcut: None, secondary_shortcut: None },
    CommandSpec { id: "insert.textBox", label: "Text Box", category: CommandCategory::Insert, default_shortcut: None, secondary_shortcut: None },
    CommandSpec { id: "insert.link", label: "Link…", category: CommandCategory::Insert, default_shortcut: Some("Cmd+K"), secondary_shortcut: None },
    // Format
    CommandSpec { id: "home.bold", label: "Bold", category: CommandCategory::Format, default_shortcut: Some("Cmd+B"), secondary_shortcut: None },
    CommandSpec { id: "home.italic", label: "Italic", category: CommandCategory::Format, default_shortcut: Some("Cmd+I"), secondary_shortcut: None },
    CommandSpec {
        id: "home.underline",
        label: "Underline",
        category: CommandCategory::Format,
        default_shortcut: Some("Cmd+U"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "home.strikethrough",
        label: "Strikethrough",
        category: CommandCategory::Format,
        default_shortcut: None,
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "home.alignLeft",
        label: "Align Left",
        category: CommandCategory::Format,
        default_shortcut: Some("Alt+Cmd+L"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "home.alignCenter",
        label: "Center",
        category: CommandCategory::Format,
        default_shortcut: Some("Alt+Cmd+C"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "home.alignRight",
        label: "Align Right",
        category: CommandCategory::Format,
        default_shortcut: Some("Alt+Cmd+R"),
        secondary_shortcut: None,
    },
    CommandSpec { id: "home.mergeCells", label: "Merge Cells", category: CommandCategory::Format, default_shortcut: None, secondary_shortcut: None },
    CommandSpec { id: "home.cellStyle", label: "Cell Styles", category: CommandCategory::Format, default_shortcut: None, secondary_shortcut: None },
    CommandSpec { id: "sheet.rename", label: "Rename Sheet", category: CommandCategory::Format, default_shortcut: None, secondary_shortcut: None },
    CommandSpec { id: "sheet.tabColor", label: "Tab Color", category: CommandCategory::Format, default_shortcut: None, secondary_shortcut: None },
    CommandSpec { id: "sheet.hide", label: "Hide Sheet", category: CommandCategory::Format, default_shortcut: None, secondary_shortcut: None },
    CommandSpec { id: "sheet.unhide", label: "Unhide Sheet…", category: CommandCategory::Format, default_shortcut: None, secondary_shortcut: None },
    // Formulas
    CommandSpec {
        id: "formulas.autoSum",
        label: "AutoSum",
        category: CommandCategory::Formulas,
        default_shortcut: Some("Alt+="),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "formulas.insertFunction",
        label: "Insert Function…",
        category: CommandCategory::Formulas,
        default_shortcut: Some("Shift+F3"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "formulas.defineName",
        label: "Define Name…",
        category: CommandCategory::Formulas,
        default_shortcut: Some("Ctrl+F3"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "formulas.nameManager",
        label: "Name Manager",
        category: CommandCategory::Formulas,
        default_shortcut: None,
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "formulas.calculateNow",
        label: "Calculate Now",
        category: CommandCategory::Formulas,
        default_shortcut: Some("F9"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "formulas.calculateSheet",
        label: "Calculate Sheet",
        category: CommandCategory::Formulas,
        default_shortcut: Some("Shift+F9"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "formulas.showFormulas",
        label: "Show Formulas",
        category: CommandCategory::Formulas,
        default_shortcut: Some("Ctrl+`"),
        secondary_shortcut: None,
    },
    // Data
    CommandSpec { id: "data.sortAscending", label: "Sort A to Z", category: CommandCategory::Data, default_shortcut: None, secondary_shortcut: None },
    CommandSpec {
        id: "data.sortDescending",
        label: "Sort Z to A",
        category: CommandCategory::Data,
        default_shortcut: None,
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "data.filter",
        label: "Filter",
        category: CommandCategory::Data,
        default_shortcut: Some("Cmd+Shift+F"),
        secondary_shortcut: None,
    },
    CommandSpec { id: "data.clearFilter", label: "Clear Filter", category: CommandCategory::Data, default_shortcut: None, secondary_shortcut: None },
    CommandSpec {
        id: "data.group",
        label: "Group…",
        category: CommandCategory::Data,
        default_shortcut: Some("Alt+Shift+Right"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "data.ungroup",
        label: "Ungroup…",
        category: CommandCategory::Data,
        default_shortcut: Some("Alt+Shift+Left"),
        secondary_shortcut: None,
    },
    // Window
    CommandSpec { id: "view.newWindow", label: "New Window", category: CommandCategory::Window, default_shortcut: None, secondary_shortcut: None },
    CommandSpec { id: "view.arrangeAll", label: "Arrange All", category: CommandCategory::Window, default_shortcut: None, secondary_shortcut: None },
    CommandSpec { id: "view.hideWindow", label: "Hide", category: CommandCategory::Window, default_shortcut: None, secondary_shortcut: None },
    CommandSpec { id: "view.unhideWindow", label: "Unhide…", category: CommandCategory::Window, default_shortcut: None, secondary_shortcut: None },
    CommandSpec {
        id: "window.activate",
        label: "Switch Windows",
        category: CommandCategory::Window,
        default_shortcut: None,
        secondary_shortcut: None,
    },
];

pub fn find_command(id: &str) -> Option<&'static CommandSpec> {
    COMMAND_REGISTRY.iter().find(|cmd| cmd.id == id)
}

pub fn commands_by_category(category: CommandCategory) -> Vec<&'static CommandSpec> {
    COMMAND_REGISTRY.iter().filter(|cmd| cmd.category == category).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_command_ids_are_unique() {
        let mut ids = HashSet::new();
        for cmd in COMMAND_REGISTRY {
            assert!(ids.insert(cmd.id), "Duplicate command ID detected: {}", cmd.id);
        }
    }

    #[test]
    fn test_lookup_finds_all_commands() {
        for cmd in COMMAND_REGISTRY {
            let found = find_command(cmd.id);
            assert!(found.is_some());
            assert_eq!(found.map(|c| c.label), Some(cmd.label));
        }
    }

    #[test]
    fn test_categories_populated() {
        assert!(!commands_by_category(CommandCategory::File).is_empty());
        assert!(!commands_by_category(CommandCategory::Edit).is_empty());
        assert!(!commands_by_category(CommandCategory::View).is_empty());
        assert!(!commands_by_category(CommandCategory::Insert).is_empty());
        assert!(!commands_by_category(CommandCategory::Format).is_empty());
        assert!(!commands_by_category(CommandCategory::Formulas).is_empty());
        assert!(!commands_by_category(CommandCategory::Data).is_empty());
        assert!(!commands_by_category(CommandCategory::Window).is_empty());
    }

    #[test]
    fn test_registry_ids_exist_in_engine() {
        for cmd in COMMAND_REGISTRY {
            assert!(gridcraft_engine::find_command(cmd.id).is_some(), "Registry id not implemented by the engine: {}", cmd.id);
        }
    }
}
