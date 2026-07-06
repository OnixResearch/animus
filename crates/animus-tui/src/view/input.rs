// Pure prompt input reducer: multiline editor, completion, slash-command detection,
// and bounded input history.
//
// Input state is advanced by applying key/command events to a pure reducer;
// terminal reads, raw-mode, and event polling stay in the shell.

/// Maximum number of input history entries to retain.
const MAX_HISTORY_SIZE: usize = 64;
/// Maximum length of a single input line.
const MAX_INPUT_LENGTH: usize = 4096;
/// Maximum completion candidates.
const MAX_COMPLETION_CANDIDATES: usize = 32;
/// Slash-command prefix character.
const SLASH_COMMAND_PREFIX: char = '/';

// ---------------------------------------------------------------------------
// Prompt input state
// ---------------------------------------------------------------------------

/// Pure prompt input state advanced by applying key/command events.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PromptInputState {
    /// Current input buffer content.
    content: String,
    /// Cursor position as byte offset into content.
    cursor: usize,
    /// Input history ring buffer (most recent first).
    history: Vec<String>,
    /// Index into history for browsing (-1 when not browsing).
    history_index: isize,
    /// Undo stack for the current input.
    undo_stack: Vec<String>,
    /// Maximum input length.
    max_length: usize,
    /// Maximum history entries.
    max_history: usize,
}

impl PromptInputState {
    /// Builds an empty prompt input state.
    #[must_use]
    pub fn new() -> Self {
        Self {
            content: String::new(),
            cursor: 0,
            history: Vec::new(),
            history_index: -1,
            undo_stack: Vec::new(),
            max_length: MAX_INPUT_LENGTH,
            max_history: MAX_HISTORY_SIZE,
        }
    }

    /// Returns the current input content.
    #[must_use]
    pub fn content(&self) -> &str {
        self.content.as_str()
    }

    /// Returns the cursor position as a byte offset.
    #[must_use]
    pub const fn cursor(&self) -> usize {
        self.cursor
    }

    /// Returns true when the input is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }

    /// Returns true when the input is not empty.
    #[must_use]
    pub fn is_nonempty(&self) -> bool {
        !self.content.is_empty()
    }

    /// Returns the input history.
    #[must_use]
    pub fn history(&self) -> &[String] {
        self.history.as_slice()
    }

    /// Returns true when the input starts with a slash (potential command).
    #[must_use]
    pub fn starts_with_slash(&self) -> bool {
        self.content.starts_with(SLASH_COMMAND_PREFIX)
    }

    /// Inserts a character at the cursor position.
    pub fn type_char(&mut self, c: char) {
        self.save_undo();
        if self.content.len() >= self.max_length {
            return;
        }
        let cursor = self.cursor.min(self.content.len());
        self.content.insert(cursor, c);
        self.cursor = cursor.saturating_add(c.len_utf8());
        self.history_index = -1;
    }

    /// Deletes the character before the cursor (backspace).
    pub fn backspace(&mut self) {
        if self.cursor == 0 || self.content.is_empty() {
            return;
        }
        self.save_undo();
        let cursor = self.cursor.min(self.content.len());
        let prev = self.content[..cursor].char_indices().next_back();
        if let Some((pos, _ch)) = prev {
            self.content.drain(pos..cursor);
            self.cursor = pos;
        }
        self.history_index = -1;
    }

    /// Deletes the character at the cursor (delete).
    pub fn delete(&mut self) {
        let cursor = self.cursor.min(self.content.len());
        if cursor >= self.content.len() {
            return;
        }
        self.save_undo();
        let next = self.content[cursor..].char_indices().next();
        if let Some((_pos, ch)) = next {
            let end = cursor + ch.len_utf8();
            self.content.drain(cursor..end);
        }
        self.history_index = -1;
    }

    /// Moves the cursor left by one character.
    pub fn move_left(&mut self) {
        if self.cursor == 0 {
            return;
        }
        let cursor = self.cursor.min(self.content.len());
        let prev = self.content[..cursor].char_indices().next_back();
        if let Some((pos, _ch)) = prev {
            self.cursor = pos;
        }
    }

    /// Moves the cursor right by one character.
    pub fn move_right(&mut self) {
        let cursor = self.cursor.min(self.content.len());
        if cursor >= self.content.len() {
            return;
        }
        let next = self.content[cursor..].char_indices().next();
        if let Some((_pos, ch)) = next {
            self.cursor = cursor + ch.len_utf8();
        }
    }

    /// Moves the cursor to the start of the input.
    pub fn move_home(&mut self) {
        self.cursor = 0;
    }

    /// Moves the cursor to the end of the input.
    pub fn move_end(&mut self) {
        self.cursor = self.content.len();
    }

    /// Submits the current input, adding it to history and clearing the buffer.
    /// Returns the submitted content.
    #[must_use]
    pub fn submit(&mut self) -> String {
        let submitted = self.content.clone();
        if !submitted.is_empty() {
            self.history.insert(0, submitted.clone());
            if self.history.len() > self.max_history {
                self.history.pop();
            }
        }
        self.content.clear();
        self.cursor = 0;
        self.history_index = -1;
        self.undo_stack.clear();
        submitted
    }

    /// Undoes the last input change.
    pub fn undo(&mut self) {
        if let Some(previous) = self.undo_stack.pop() {
            self.content = previous;
            self.cursor = self.content.len();
        }
    }

    /// Navigates backward through input history.
    pub fn history_up(&mut self) {
        if self.history.is_empty() {
            return;
        }
        let next_index = self.history_index.saturating_add(1);
        #[allow(clippy::cast_sign_loss)]
        let idx = next_index as usize;
        if let Some(entry) = self.history.get(idx).cloned() {
            self.save_undo();
            self.history_index = next_index;
            self.content = entry;
            self.cursor = self.content.len();
        }
    }

    /// Navigates forward through input history.
    pub fn history_down(&mut self) {
        if self.history_index < 0 {
            return;
        }
        let prev_index = self.history_index - 1;
        self.save_undo();
        if prev_index < 0 {
            self.content.clear();
            self.cursor = 0;
            self.history_index = -1;
        } else {
            self.history_index = prev_index;
            #[allow(clippy::cast_sign_loss)]
            let idx = prev_index as usize;
            if let Some(entry) = self.history.get(idx).cloned() {
                self.content = entry;
                self.cursor = self.content.len();
            }
        }
    }

    /// Saves the current content to the undo stack.
    fn save_undo(&mut self) {
        self.undo_stack.push(self.content.clone());
    }
}

impl Default for PromptInputState {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Completion menu
// ---------------------------------------------------------------------------

/// A completion candidate with a label and optional description.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompletionCandidate {
    /// Display label.
    pub label: String,
    /// Optional description.
    pub description: String,
}

/// Bounded, deterministically ordered completion menu.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompletionMenu {
    /// Current candidates.
    candidates: Vec<CompletionCandidate>,
    /// Index of the selected candidate, if any.
    selected: Option<usize>,
    /// Maximum number of candidates.
    max_candidates: usize,
}

impl CompletionMenu {
    /// Builds an empty completion menu.
    #[must_use]
    pub fn new() -> Self {
        Self {
            candidates: Vec::new(),
            selected: None,
            max_candidates: MAX_COMPLETION_CANDIDATES,
        }
    }

    /// Populates candidates from a sorted list, bounded to the max.
    pub fn populate(&mut self, candidates: Vec<CompletionCandidate>) {
        self.candidates = candidates;
        self.candidates.truncate(self.max_candidates);
        self.selected = if self.candidates.is_empty() {
            None
        } else {
            Some(0)
        };
    }

    /// Clears all candidates.
    pub fn clear(&mut self) {
        self.candidates.clear();
        self.selected = None;
    }

    /// Returns the current candidates.
    #[must_use]
    pub fn candidates(&self) -> &[CompletionCandidate] {
        self.candidates.as_slice()
    }

    /// Returns the selected index, if any.
    #[must_use]
    pub const fn selected_index(&self) -> Option<usize> {
        self.selected
    }

    /// Returns true when there are candidates.
    #[must_use]
    pub fn has_candidates(&self) -> bool {
        !self.candidates.is_empty()
    }

    /// Selects the next candidate (wrapping).
    pub fn select_next(&mut self) {
        if self.candidates.is_empty() {
            return;
        }
        let next = self.selected.map_or(0, |i| {
            if i + 1 >= self.candidates.len() {
                0
            } else {
                i + 1
            }
        });
        self.selected = Some(next);
    }

    /// Selects the previous candidate (wrapping).
    pub fn select_prev(&mut self) {
        if self.candidates.is_empty() {
            return;
        }
        let prev = self.selected.map_or(self.candidates.len() - 1, |i| {
            if i == 0 {
                self.candidates.len() - 1
            } else {
                i - 1
            }
        });
        self.selected = Some(prev);
    }

    /// Returns the selected candidate label, if any.
    #[must_use]
    pub fn selected_label(&self) -> Option<&str> {
        self.selected
            .and_then(|i| self.candidates.get(i))
            .map(|c| c.label.as_str())
    }
}

impl Default for CompletionMenu {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Slash-command detection
// ---------------------------------------------------------------------------

/// A detected slash command with its name and arguments.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SlashCommand {
    /// Command name (without leading `/`).
    pub name: String,
    /// Remaining text after the command name.
    pub args: String,
    /// Whether the command is known (matched against manifests).
    pub known: bool,
}

/// Detects and parses slash commands from prompt input.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SlashCommandDetector {
    /// Known command names (populated from manifests).
    known_commands: Vec<String>,
}

impl SlashCommandDetector {
    /// Builds a new detector with the given known command names.
    #[must_use]
    pub fn new(known_commands: Vec<String>) -> Self {
        Self { known_commands }
    }

    /// Detects a slash command from input text.
    ///
    /// Returns `None` when the input does not start with `/`.
    #[must_use]
    pub fn detect(&self, input: &str) -> Option<SlashCommand> {
        if !input.starts_with(SLASH_COMMAND_PREFIX) {
            return None;
        }
        let rest = &input[SLASH_COMMAND_PREFIX.len_utf8()..];
        let mut parts = rest.splitn(2, |c: char| c.is_whitespace());
        let name = parts.next().unwrap_or("").to_string();
        let args = parts.next().unwrap_or("").to_string();
        let known = self.known_commands.iter().any(|k| k == &name);
        Some(SlashCommand {
            name,
            args,
            known,
        })
    }

    /// Returns the list of known command names.
    #[must_use]
    pub fn known_commands(&self) -> &[String] {
        self.known_commands.as_slice()
    }
}

#[cfg(test)]
mod input_tests {
    use super::*;

    // -----------------------------------------------------------------------
    // Prompt input state tests
    // -----------------------------------------------------------------------

    #[test]
    fn empty_input_is_empty() {
        let state = PromptInputState::new();
        assert!(state.is_empty());
        assert_eq!(state.content(), "");
        assert_eq!(state.cursor(), 0);
    }

    #[test]
    fn type_char_appends_content() {
        let mut state = PromptInputState::new();
        state.type_char('h');
        state.type_char('i');
        assert_eq!(state.content(), "hi");
        assert_eq!(state.cursor(), 2);
    }

    #[test]
    fn type_char_inserts_at_cursor() {
        let mut state = PromptInputState::new();
        state.type_char('a');
        state.type_char('c');
        state.move_left();
        state.type_char('b');
        assert_eq!(state.content(), "abc");
        assert_eq!(state.cursor(), 2); // after 'b'
    }

    #[test]
    fn backspace_removes_character_before_cursor() {
        let mut state = PromptInputState::new();
        state.type_char('a');
        state.type_char('b');
        state.type_char('c');
        state.backspace();
        assert_eq!(state.content(), "ab");
        assert_eq!(state.cursor(), 2);
    }

    #[test]
    fn backspace_at_start_does_nothing() {
        let mut state = PromptInputState::new();
        state.type_char('a');
        state.move_home();
        state.backspace();
        assert_eq!(state.content(), "a");
    }

    #[test]
    fn delete_removes_character_at_cursor() {
        let mut state = PromptInputState::new();
        state.type_char('a');
        state.type_char('b');
        state.type_char('c');
        state.move_home();
        state.move_right(); // cursor at 'b'
        state.delete();
        assert_eq!(state.content(), "ac");
    }

    #[test]
    fn submit_clears_content_and_adds_to_history() {
        let mut state = PromptInputState::new();
        state.type_char('h');
        state.type_char('i');
        let submitted = state.submit();
        assert_eq!(submitted, "hi");
        assert!(state.is_empty());
        assert_eq!(state.history().len(), 1);
        assert_eq!(state.history()[0], "hi");
    }

    #[test]
    fn empty_submit_does_not_add_to_history() {
        let mut state = PromptInputState::new();
        let submitted = state.submit();
        assert!(submitted.is_empty());
        assert!(state.history().is_empty());
    }

    #[test]
    fn history_navigation_restores_previous_inputs() {
        let mut state = PromptInputState::new();
        state.type_char('a');
        let _ = state.submit();
        state.type_char('b');
        let _ = state.submit();
        state.type_char('c');
        // Before submitting c, browse history
        state.history_up();
        assert_eq!(state.content(), "b");
        state.history_up();
        assert_eq!(state.content(), "a");
        state.history_down();
        assert_eq!(state.content(), "b");
        state.history_down();
        assert!(state.is_empty());
    }

    #[test]
    fn undo_restores_previous_content() {
        let mut state = PromptInputState::new();
        state.type_char('a');
        state.type_char('b');
        state.type_char('c');
        state.backspace(); // removes 'c'
        state.undo();
        assert_eq!(state.content(), "abc");
    }

    #[test]
    fn move_home_and_end() {
        let mut state = PromptInputState::new();
        state.type_char('a');
        state.type_char('b');
        state.move_home();
        assert_eq!(state.cursor(), 0);
        state.move_end();
        assert_eq!(state.cursor(), 2);
    }

    #[test]
    fn max_length_limits_input() {
        let mut state = PromptInputState::new();
        for _ in 0..MAX_INPUT_LENGTH + 10 {
            state.type_char('x');
        }
        assert_eq!(state.content().len(), MAX_INPUT_LENGTH);
    }

    // -----------------------------------------------------------------------
    // Completion menu tests
    // -----------------------------------------------------------------------

    #[test]
    fn empty_completion_menu() {
        let menu = CompletionMenu::new();
        assert!(!menu.has_candidates());
        assert!(menu.candidates().is_empty());
        assert!(menu.selected_index().is_none());
    }

    #[test]
    fn populate_adds_candidates() {
        let mut menu = CompletionMenu::new();
        menu.populate(vec![
            CompletionCandidate {
                label: String::from("/help"),
                description: String::from("Show help"),
            },
            CompletionCandidate {
                label: String::from("/quit"),
                description: String::from("Quit"),
            },
        ]);
        assert!(menu.has_candidates());
        assert_eq!(menu.candidates().len(), 2);
        assert_eq!(menu.selected_index(), Some(0));
    }

    #[test]
    fn select_next_prev_wraps() {
        let mut menu = CompletionMenu::new();
        menu.populate(vec![
            CompletionCandidate {
                label: String::from("a"),
                description: String::default(),
            },
            CompletionCandidate {
                label: String::from("b"),
                description: String::default(),
            },
        ]);
        menu.select_next();
        assert_eq!(menu.selected_index(), Some(1));
        menu.select_next();
        assert_eq!(menu.selected_index(), Some(0)); // wraps
        menu.select_prev();
        assert_eq!(menu.selected_index(), Some(1)); // wraps
    }

    #[test]
    fn populate_truncates_to_max() {
        let mut menu = CompletionMenu::new();
        let many: Vec<_> = (0..MAX_COMPLETION_CANDIDATES + 10)
            .map(|i| CompletionCandidate {
                label: format!("candidate_{i}"),
                description: String::default(),
            })
            .collect();
        menu.populate(many);
        assert_eq!(menu.candidates().len(), MAX_COMPLETION_CANDIDATES);
    }

    // -----------------------------------------------------------------------
    // Slash-command detection tests
    // -----------------------------------------------------------------------

    #[test]
    fn non_slash_input_returns_none() {
        let detector = SlashCommandDetector::new(vec![]);
        assert!(detector.detect("hello").is_none());
        assert!(detector.detect("").is_none());
    }

    #[test]
    fn known_command_is_detected() {
        let detector = SlashCommandDetector::new(vec![
            String::from("help"),
            String::from("quit"),
        ]);
        let cmd = detector.detect("/help").expect("should detect");
        assert_eq!(cmd.name, "help");
        assert!(cmd.known);
        assert!(cmd.args.is_empty());
    }

    #[test]
    fn unknown_command_is_marked() {
        let detector = SlashCommandDetector::new(vec![]);
        let cmd = detector.detect("/unknown").expect("should detect");
        assert_eq!(cmd.name, "unknown");
        assert!(!cmd.known);
    }

    #[test]
    fn command_with_args_parses() {
        let detector = SlashCommandDetector::new(vec![]);
        let cmd = detector.detect("/run test --verbose").expect("should detect");
        assert_eq!(cmd.name, "run");
        assert_eq!(cmd.args, "test --verbose");
    }

    #[test]
    fn empty_slash_returns_empty_name() {
        let detector = SlashCommandDetector::new(vec![]);
        let cmd = detector.detect("/").expect("should detect");
        assert!(cmd.name.is_empty());
        assert!(!cmd.known);
    }

    // -----------------------------------------------------------------------
    // Negative tests
    // -----------------------------------------------------------------------

    #[test]
    fn malformed_slash_input_does_not_panic() {
        let detector = SlashCommandDetector::new(vec![]);
        // Various edge cases
        assert!(detector.detect("/").is_some());
        assert!(detector.detect("//").is_some());
        assert!(detector.detect("/  ").is_some());
        // Non-slash inputs
        assert!(detector.detect("nope").is_none());
        assert!(detector.detect("").is_none());
    }

        #[test]
    fn unbounded_history_growth_is_prevented() {
        const OVERFLOW_TEST_ENTRIES: usize = 10;
        let mut state = PromptInputState::new();
        let limit = MAX_HISTORY_SIZE.saturating_add(OVERFLOW_TEST_ENTRIES);
        for i in 0..limit {
            let content = format!("input {i}");
            for c in content.chars() {
                state.type_char(c);
            }
            let _ = state.submit();
        }
        assert!(state.history().len() <= MAX_HISTORY_SIZE);
    }

    #[test]
    fn unknown_command_in_completion_not_crash() {
        let mut menu = CompletionMenu::new();
        // No candidates populated
        menu.select_next(); // should not panic
        menu.select_prev(); // should not panic
        assert!(menu.selected_label().is_none());
    }

    #[test]
    fn stale_completion_after_clear() {
        let mut menu = CompletionMenu::new();
        menu.populate(vec![
            CompletionCandidate {
                label: String::from("/help"),
                description: String::default(),
            },
        ]);
        assert!(menu.has_candidates());
        menu.clear();
        assert!(!menu.has_candidates());
        assert!(menu.selected_index().is_none());
    }
}