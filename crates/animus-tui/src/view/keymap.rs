// Host-only keymap binding layer for the prompt input.
//
// This module uses crossterm key-event types directly. Reusable crates
// never import these types.

use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;
use crossterm::event::KeyModifiers;

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

/// A keybinding mapping a crossterm key event to an input action.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Keybinding {
    /// The key event to match.
    pub key: KeyEvent,
    /// The action to trigger.
    pub action: InputAction,
}

/// Default keybindings for the prompt input.
///
/// Returns a list of keybindings mapping common keys to input actions.
#[must_use]
pub fn default_keybindings() -> Vec<Keybinding> {
    vec![
        // Movement
        Keybinding {
            key: KeyCode::Left.into(),
            action: InputAction::MoveLeft,
        },
        Keybinding {
            key: KeyCode::Right.into(),
            action: InputAction::MoveRight,
        },
        Keybinding {
            key: KeyCode::Home.into(),
            action: InputAction::MoveHome,
        },
        Keybinding {
            key: KeyCode::End.into(),
            action: InputAction::MoveEnd,
        },
        // Editing
        Keybinding {
            key: KeyCode::Backspace.into(),
            action: InputAction::Backspace,
        },
        Keybinding {
            key: KeyCode::Delete.into(),
            action: InputAction::Delete,
        },
        // Submit
        Keybinding {
            key: KeyCode::Enter.into(),
            action: InputAction::Submit,
        },
        // History
        Keybinding {
            key: KeyCode::Up.into(),
            action: InputAction::HistoryUp,
        },
        Keybinding {
            key: KeyCode::Down.into(),
            action: InputAction::HistoryDown,
        },
        // Undo
        Keybinding {
            key: KeyEvent::new(KeyCode::Char('z'), KeyModifiers::CONTROL),
            action: InputAction::Undo,
        },
        // Completion
        Keybinding {
            key: KeyCode::Tab.into(),
            action: InputAction::Complete,
        },
        // Cancel
        Keybinding {
            key: KeyCode::Esc.into(),
            action: InputAction::Cancel,
        },
    ]
}

/// Resolves a crossterm key event to an input action using the given keybindings.
#[must_use]
pub fn resolve_keybinding(key_event: &KeyEvent, bindings: &[Keybinding]) -> InputAction {
    for binding in bindings {
        if binding.key == *key_event {
            return binding.action;
        }
    }
    InputAction::Noop
}

/// Resolves a crossterm key event to an input action using default keybindings.
#[must_use]
pub fn resolve_default_keybinding(key_event: &KeyEvent) -> InputAction {
    resolve_keybinding(key_event, &default_keybindings())
}

#[cfg(test)]
mod keymap_tests {
    use super::*;
    use crossterm::event::KeyCode;
    use crossterm::event::KeyEvent;
    use crossterm::event::KeyModifiers;

    #[test]
    fn default_bindings_map_enter_to_submit() {
        let action = resolve_default_keybinding(&KeyCode::Enter.into());
        assert_eq!(action, InputAction::Submit);
    }

    #[test]
    fn default_bindings_map_left_to_move_left() {
        let action = resolve_default_keybinding(&KeyCode::Left.into());
        assert_eq!(action, InputAction::MoveLeft);
    }

    #[test]
    fn default_bindings_map_backspace() {
        let action = resolve_default_keybinding(&KeyCode::Backspace.into());
        assert_eq!(action, InputAction::Backspace);
    }

    #[test]
    fn default_bindings_map_escape_to_cancel() {
        let action = resolve_default_keybinding(&KeyCode::Esc.into());
        assert_eq!(action, InputAction::Cancel);
    }

    #[test]
    fn ctrl_z_maps_to_undo() {
        let key_event = KeyEvent::new(KeyCode::Char('z'), KeyModifiers::CONTROL);
        let action = resolve_default_keybinding(&key_event);
        assert_eq!(action, InputAction::Undo);
    }

    #[test]
    fn unknown_key_maps_to_noop() {
        let action = resolve_default_keybinding(&KeyCode::F(1).into());
        assert_eq!(action, InputAction::Noop);
    }

    #[test]
    fn custom_binding_overrides_default() {
        let bindings = vec![Keybinding {
            key: KeyCode::Enter.into(),
            action: InputAction::Noop,
        }];
        let action = resolve_keybinding(&KeyCode::Enter.into(), &bindings);
        assert_eq!(action, InputAction::Noop);
    }
}