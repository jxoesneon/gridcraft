//! Keystroke state machine providing Excel keyboard ergonomics.
//!
//! Letters type into the active cell, so tool keys are non-text keys only:
//! Space and Z are spring-loaded navigation tools (held, not latched), F2
//! starts in-cell editing and Escape returns to the Select tool. Gating
//! letter input while `editing` is the shell's job.

use gridcraft_engine::Tool;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KeyModifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub cmd: bool,
}

impl KeyModifiers {
    pub const fn empty() -> Self {
        Self { shift: false, ctrl: false, alt: false, cmd: false }
    }
}

pub struct KeyboardEngine {
    pub prior_tool: Option<Tool>,
    pub space_held: bool,
    pub z_held: bool,
    pub alt_held: bool,
}

impl KeyboardEngine {
    pub fn new() -> Self {
        Self { prior_tool: None, space_held: false, z_held: false, alt_held: false }
    }

    pub fn on_key_down(&mut self, key: &str, current: Tool) -> Option<Tool> {
        match key {
            "Space" if !self.space_held => {
                self.space_held = true;
                self.prior_tool = Some(current);
                Some(Tool::Hand)
            }
            "z" | "Z" if !self.z_held => {
                self.z_held = true;
                self.prior_tool = Some(current);
                Some(Tool::Zoom)
            }
            "Alt" => {
                self.alt_held = true;
                None
            }
            // Non-text tool keys (letters belong to cell entry)
            "F2" => Some(Tool::EditCell),
            "Escape" => Some(Tool::Select),
            "Insert" => Some(Tool::Insert),
            "h" | "H" => Some(Tool::Hand),
            _ => None,
        }
    }

    pub fn on_key_up(&mut self, key: &str) -> Option<Tool> {
        match key {
            "Space" if self.space_held => {
                self.space_held = false;
                self.prior_tool.take()
            }
            "z" | "Z" if self.z_held => {
                self.z_held = false;
                self.prior_tool.take()
            }
            "Alt" => {
                self.alt_held = false;
                None
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_keys() {
        let mut k = KeyboardEngine::new();
        assert_eq!(k.on_key_down("F2", Tool::Select), Some(Tool::EditCell));
        assert_eq!(k.on_key_down("Escape", Tool::EditCell), Some(Tool::Select));
        assert_eq!(k.on_key_down("Insert", Tool::Select), Some(Tool::Insert));
        assert_eq!(k.on_key_down("h", Tool::Select), Some(Tool::Hand));
        assert_eq!(k.on_key_down("q", Tool::Select), None); // letter: cell entry
    }

    #[test]
    fn test_spring_loaded_hand_tool() {
        let mut k = KeyboardEngine::new();
        let initial = Tool::Select;

        // Press Space: temporary Hand
        assert_eq!(k.on_key_down("Space", initial), Some(Tool::Hand));
        assert!(k.space_held);

        // Multiple down events shouldn't overwrite prior tool
        assert_eq!(k.on_key_down("Space", Tool::Hand), None);

        // Release Space: restores initial tool
        assert_eq!(k.on_key_up("Space"), Some(initial));
        assert!(!k.space_held);
    }

    #[test]
    fn test_spring_loaded_zoom_tool() {
        let mut k = KeyboardEngine::new();
        let initial = Tool::Format;

        assert_eq!(k.on_key_down("z", initial), Some(Tool::Zoom));
        assert!(k.z_held);

        assert_eq!(k.on_key_up("z"), Some(initial));
        assert!(!k.z_held);
    }
}
