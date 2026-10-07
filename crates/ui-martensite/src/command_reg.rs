//! Spreadsheet commands.

pub struct Command {
    pub id: &'static str,
    pub label: &'static str,
}

pub const COMMANDS: &[Command] = &[
    Command { id: "sheet.insert_row", label: "Insert Row Above" },
    Command { id: "sheet.insert_col", label: "Insert Column Left" },
];
