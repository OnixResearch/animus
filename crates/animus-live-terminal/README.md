# Animus Live Terminal

This non-default AGPL product crate owns a bounded local terminal client. It enters raw mode, uses the alternate screen, handles input and resize events, and restores terminal state.

The client uses one explicit daemon endpoint or one injected direct service. It sends typed command descriptors. It decodes bounded private runtime events after delivery. Provider, tool, Steel, session, persistence, and worker execution stay outside this crate.

The `live-terminal` feature enables the real Crossterm shell. Default builds keep terminal authority disabled.
