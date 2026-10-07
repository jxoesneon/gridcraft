//! Spreadsheet integration tests.

use gridcraft_ui_martensite::GridcraftApp;

#[test]
fn test_sheet_workflow() {
    let mut app = GridcraftApp::new();
    app.sheet.set_cell(1, 1, "42.0");
    assert_eq!(app.sheet.get_cell(1, 1), Some("42.0"));
    app.select_cell(1, 1);
    assert_eq!(app.active_cell, (1, 1));
}
