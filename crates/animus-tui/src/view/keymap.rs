// Host-only keymap binding layer for the prompt input.
//
// Key parsing and event matching go through Subwayrat's rat-keymap crate.
// Reusable crates never import terminal key-event types.

const KEY_LEFT: &str = "Left";
const KEY_RIGHT: &str = "Right";
const KEY_HOME: &str = "Home";
const KEY_END: &str = "End";
const KEY_BACKSPACE: &str = "Backspace";
const KEY_DELETE: &str = "Delete";
const KEY_ENTER: &str = "Enter";
const KEY_UP: &str = "Up";
const KEY_DOWN: &str = "Down";
const KEY_UNDO: &str = "Ctrl+z";
const KEY_TAB: &str = "Tab";
const KEY_ESCAPE: &str = "Esc";

/// Input action triggered by a keybinding.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InputAction {
    /// Insert a character at the cursor.
    Insert(char),
    /// Delete the character before the cursor.
    Backspace,
    /// Delete the character at the cursor.
    Delete,
    /// Move cursor left.
    MoveLeft,
    /// Move cursor right.
    MoveRight,
    /// Move cursor to start of input.
    MoveHome,
    /// Move cursor to end of input.
    MoveEnd,
    /// Submit the current input.
    Submit,
    /// Undo the last change.
    Undo,
    /// Navigate backward through history.
    HistoryUp,
    /// Navigate forward through history.
    HistoryDown,
    /// Open the completion menu.
    Complete,
    /// Select next completion candidate.
    CompleteNext,
    /// Select previous completion candidate.
    CompletePrev,
    /// Accept the selected completion.
    CompleteAccept,
    /// Cancel the current action.
    Cancel,
    /// No-op keybinding.
    Noop,
}

/// A keybinding mapping a Subwayrat key chord to an input action.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Keybinding {
    /// The key chord to match.
    pub key: rat_keymap::KeyCombo,
    /// The action to trigger.
    pub action: InputAction,
}

/// Default keybindings for the prompt input.
///
/// Returns a list of keybindings mapping common keys to input actions.
#[must_use]
pub fn default_keybindings() -> Vec<Keybinding> {
    vec![
        default_binding(KEY_LEFT, InputAction::MoveLeft),
        default_binding(KEY_RIGHT, InputAction::MoveRight),
        default_binding(KEY_HOME, InputAction::MoveHome),
        default_binding(KEY_END, InputAction::MoveEnd),
        default_binding(KEY_BACKSPACE, InputAction::Backspace),
        default_binding(KEY_DELETE, InputAction::Delete),
        default_binding(KEY_ENTER, InputAction::Submit),
        default_binding(KEY_UP, InputAction::HistoryUp),
        default_binding(KEY_DOWN, InputAction::HistoryDown),
        default_binding(KEY_UNDO, InputAction::Undo),
        default_binding(KEY_TAB, InputAction::Complete),
        default_binding(KEY_ESCAPE, InputAction::Cancel),
    ]
}

/// Resolves a Subwayrat key chord to an input action using the given keybindings.
#[must_use]
pub fn resolve_keybinding(
    key_combo: &rat_keymap::KeyCombo,
    bindings: &[Keybinding],
) -> InputAction {
    bindings
        .iter()
        .find(|binding| binding.key == *key_combo)
        .map_or(InputAction::Noop, |binding| binding.action)
}

/// Resolves a Subwayrat key chord to an input action using default keybindings.
#[must_use]
pub fn resolve_default_keybinding(key_combo: &rat_keymap::KeyCombo) -> InputAction {
    resolve_keybinding(key_combo, &default_keybindings())
}

fn default_binding(key: &'static str, action: InputAction) -> Keybinding {
    Keybinding {
        key: parse_default_key(key),
        action,
    }
}

fn parse_default_key(key: &'static str) -> rat_keymap::KeyCombo {
    rat_keymap::parse_key_string(key).expect("default Subwayrat keybinding must parse")
}

#[cfg(test)]
mod keymap_tests {
    use super::*;

    const KEY_UNKNOWN: &str = "F1";

    #[test]
    fn default_bindings_map_enter_to_submit() {
        let action = resolve_default_keybinding(&parse_default_key(KEY_ENTER));
        assert_eq!(action, InputAction::Submit);
    }

    #[test]
    fn default_bindings_map_left_to_move_left() {
        let action = resolve_default_keybinding(&parse_default_key(KEY_LEFT));
        assert_eq!(action, InputAction::MoveLeft);
    }

    #[test]
    fn default_bindings_map_backspace() {
        let action = resolve_default_keybinding(&parse_default_key(KEY_BACKSPACE));
        assert_eq!(action, InputAction::Backspace);
    }

    #[test]
    fn default_bindings_map_escape_to_cancel() {
        let action = resolve_default_keybinding(&parse_default_key(KEY_ESCAPE));
        assert_eq!(action, InputAction::Cancel);
    }

    #[test]
    fn ctrl_z_maps_to_undo() {
        let action = resolve_default_keybinding(&parse_default_key(KEY_UNDO));
        assert_eq!(action, InputAction::Undo);
    }

    #[test]
    fn unknown_key_maps_to_noop() {
        let action = resolve_default_keybinding(&parse_default_key(KEY_UNKNOWN));
        assert_eq!(action, InputAction::Noop);
    }

    #[test]
    fn custom_binding_overrides_default() {
        let bindings = vec![Keybinding {
            key: parse_default_key(KEY_ENTER),
            action: InputAction::Noop,
        }];
        let action = resolve_keybinding(&parse_default_key(KEY_ENTER), &bindings);
        assert_eq!(action, InputAction::Noop);
    }
}
