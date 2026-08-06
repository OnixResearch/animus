//! Animus-owned weave node content.

use alloc::string::String;

use universal_weave::{DiscreteContentResult, DiscreteContents, IndependentContents};

/// Text content for one weave node.
///
/// The text is opaque to the adapter: it is never inspected for meaning and
/// never appears in receipts or snapshots. Splitting fails outside UTF-8
/// character boundaries. Merging concatenates the two parts in order.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct WeaveNodeContent {
    text: String,
}

impl WeaveNodeContent {
    /// Creates content from owned text.
    #[must_use]
    pub fn new(text: String) -> Self {
        Self { text }
    }

    /// Returns the text as a string slice.
    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Returns the text length in bytes.
    #[must_use]
    pub fn len(&self) -> usize {
        self.text.len()
    }

    /// Returns `true` when the text is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }
}

impl IndependentContents for WeaveNodeContent {}

impl DiscreteContents for WeaveNodeContent {
    fn split(self, at: usize) -> DiscreteContentResult<Self> {
        if at == 0 || at >= self.text.len() || !self.text.is_char_boundary(at) {
            return DiscreteContentResult::One(self);
        }
        let right = self.text.get(at..).map(String::from);
        let mut left = self.text;
        match right {
            Some(right) => {
                left.truncate(at);
                DiscreteContentResult::Two(Self { text: left }, Self { text: right })
            }
            None => DiscreteContentResult::One(Self { text: left }),
        }
    }

    fn merge(self, value: Self) -> DiscreteContentResult<Self> {
        let mut text = self.text;
        text.push_str(&value.text);
        DiscreteContentResult::One(Self { text })
    }
}
