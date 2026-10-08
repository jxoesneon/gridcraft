//! Comprehensive integration test suite for GridCraft Martensite UI.
//!
//! Validates end-to-end integration across commands, menus, shortcuts,
//! scrubby inputs, the fill handle, sheet tabs and grid coordinates —
//! and that every registered command id resolves in the engine catalog.

use gridcraft_core::{CellRef, RangeRef};
use gridcraft_engine::{Engine, Tool};
use gridcraft_ui_martensite::{
    GridcraftApp,
    command_reg::{COMMAND_REGISTRY, find_command},
    menus::generate_main_menu,
    theme::CraftTheme,
    widgets::{DockPanelGroup, FillDirection, FillHandleWidget, FormulaBarWidget, SheetTabBarWidget, SheetTabDef, SheetViewWidget},
};

#[test]
fn test_end_to_end_workspace_interaction() {
    let engine = Engine::new();
    let mut app = GridcraftApp::new(engine);

    // 1. Initial State Verification
    assert_eq!(app.active_tool, Tool::Select);
    assert_eq!(app.zoom_level, 1.0);
    assert!(app.gridlines_visible);
    assert!(!app.editing);

    // 2. Keystroke Workflow: F2 edits a cell, Space pans, Z zooms
    let new_tool = app.keyboard.on_key_down("F2", app.active_tool);
    assert_eq!(new_tool, Some(Tool::EditCell));
    app.set_tool(Tool::EditCell);
    app.begin_edit(Some("=SUM(B2:B10)".to_string()));
    assert_eq!(app.commit_edit(), "=SUM(B2:B10)");

    // Spring-loaded Hand tool
    let hand_tool = app.keyboard.on_key_down("Space", app.active_tool);
    assert_eq!(hand_tool, Some(Tool::Hand));
    app.set_tool(Tool::Hand);

    app.pan_by(50.0, 100.0);
    assert_eq!(app.pan_offset, [50.0, 100.0]);

    // Release Space restores EditCell
    let restored_tool = app.keyboard.on_key_up("Space");
    assert_eq!(restored_tool, Some(Tool::EditCell));
    app.set_tool(Tool::EditCell);
    app.set_tool(Tool::Select);

    // 3. Formula Bar: name box tracks the selection, input line edits
    app.select_range(RangeRef::new(CellRef::new(1, 1), CellRef::new(9, 1))); // B2:B10
    assert_eq!(app.selection.bounds().a1(), "B2:B10");

    let mut bar = FormulaBarWidget::new();
    bar.set_name_box(app.selection.bounds().a1());
    assert_eq!(bar.name_box, "B2:B10");
    bar.zoom.on_pointer_down(0.0);
    bar.zoom.on_pointer_move(20.0, false, false);
    bar.zoom.on_pointer_up();
    assert_eq!(bar.zoom.value, 120.0); // 100 + 20

    // 4. Sheet Tabs: activate, reorder, hide
    let mut tabs = SheetTabBarWidget::new();
    tabs.sheets = vec![
        SheetTabDef { id: 1, name: "Sales".to_string(), hidden: false, protected: false, tab_color: None },
        SheetTabDef { id: 2, name: "Budget".to_string(), hidden: false, protected: false, tab_color: None },
    ];
    tabs.select_sheet(2);
    assert_eq!(tabs.active_sheet_id, Some(2));
    tabs.reorder(2, 0);
    assert_eq!(tabs.active_index(), Some(0));

    // 5. Fill Handle: drag A1:A2 down to extend a series
    let mut fill = FillHandleWidget::new();
    fill.begin(RangeRef::new(CellRef::new(0, 0), CellRef::new(1, 0)));
    assert_eq!(fill.update(CellRef::new(7, 0)), Some(FillDirection::Down));
    assert_eq!(fill.commit().map(|r| r.a1()), Some("A1:A8".to_string()));

    // 6. Sheet View: scroll/zoom geometry and marching ants
    let mut view = SheetViewWidget::new();
    view.zoom_at(2.0, [0.0, 0.0]);
    assert_eq!(view.zoom, 2.0);
    assert_eq!(view.screen_to_cell([140.0, 40.0]), CellRef::new(1, 1)); // B2

    // 7. Docking System Validation
    let mut dock = DockPanelGroup::new(&["Format", "Functions", "Selection"]);
    dock.select_tab(2);
    assert_eq!(dock.active_tab, 2);
    dock.toggle_collapsed();
    assert!(dock.collapsed_to_icons);

    // 8. Menu Generation Consistency (menus only hold registered commands)
    let menus = generate_main_menu();
    assert!(!menus.is_empty());
    for menu in &menus {
        for item in &menu.items {
            if let Some(cmd_id) = item.command_id {
                assert!(find_command(cmd_id).is_some(), "Unknown command in menu: {}", cmd_id);
            }
        }
    }

    // 9. Registry ids are real engine commands.
    for cmd in COMMAND_REGISTRY {
        assert!(gridcraft_engine::find_command(cmd.id).is_some(), "Registry id not in the engine catalog: {}", cmd.id);
    }

    // 10. Theme Color Space Consistency
    let theme = CraftTheme::dark_neutral();
    let obsidian = CraftTheme::studio_obsidian();
    assert_ne!(theme.surface_app_bg, obsidian.surface_app_bg);
}
