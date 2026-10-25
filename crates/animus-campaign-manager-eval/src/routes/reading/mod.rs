//! Reading a rendered brief the way a scripted policy needs it: rank dimensions, listed candidates
//! with their families and observed signals, and offers.

/// Heading of the candidate list in the rendered brief.
const CANDIDATES_HEADING: &str = "Candidates (";
/// Heading of the offer list in the rendered brief.
const OFFERS_HEADING: &str = "Offers you may add:";
/// Prefix of the rank-domain line in the rendered brief.
const RANKS_PREFIX: &str = "Ranks: ";
/// Marker the host writes into an executed candidate's label.
const SIGNAL_MARKER: &str = "[signal ";
/// Separator between a family and an approach in a scenario label.
const FAMILY_SEPARATOR: char = '/';
/// Rank dimensions assumed when the brief names none.
const DEFAULT_DIMENSIONS: u32 = 1;

/// One listed candidate or offer.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct Listed {
    /// Its handle, such as `c1` or `o1`.
    pub(super) handle: String,
    /// Its label's family.
    pub(super) family: String,
    /// The signal the host observed when it ran, if it ran.
    pub(super) signal: Option<u32>,
}

/// What a policy reads from one rendered brief.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct PromptView {
    /// Rank dimensions per score.
    pub(super) dimensions: u32,
    /// Listed candidates in brief order.
    pub(super) candidates: Vec<Listed>,
    /// Offers in menu order.
    pub(super) offers: Vec<Listed>,
}

/// The list a brief line belongs to.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Section {
    Preamble,
    Candidates,
    Offers,
}

/// Reads one rendered brief.
pub(super) fn read_prompt(prompt: &str) -> PromptView {
    let mut view = PromptView {
        dimensions: DEFAULT_DIMENSIONS,
        candidates: Vec::new(),
        offers: Vec::new(),
    };
    let mut section = Section::Preamble;
    for line in prompt.lines() {
        if let Some(rest) = line.strip_prefix(RANKS_PREFIX) {
            view.dimensions = leading_number(rest).unwrap_or(DEFAULT_DIMENSIONS);
        }
        section = next_section(section, line);
        let Some(listed) = listed(line) else {
            continue;
        };
        match section {
            Section::Candidates => view.candidates.push(listed),
            Section::Offers => view.offers.push(listed),
            Section::Preamble => {}
        }
    }
    debug_assert!(view.dimensions >= DEFAULT_DIMENSIONS);
    view
}

/// The section `line` opens, or the current one.
fn next_section(current: Section, line: &str) -> Section {
    if line.starts_with(CANDIDATES_HEADING) {
        Section::Candidates
    } else if line.starts_with(OFFERS_HEADING) {
        Section::Offers
    } else {
        current
    }
}

/// The handle, family, and observed signal of one `c<n> …: label` or `o<n>: label` line.
fn listed(line: &str) -> Option<Listed> {
    let (head, label) = line.split_once(": ")?;
    let handle = head.split(' ').next().filter(|handle| is_handle(handle))?;
    let family = label.split(FAMILY_SEPARATOR).next().unwrap_or(label).trim();
    let signal = label
        .split_once(SIGNAL_MARKER)
        .and_then(|(_, rest)| leading_number(rest));
    Some(Listed {
        handle: String::from(handle),
        family: String::from(family),
        signal,
    })
}

/// Whether `token` is a canonical-looking `c<n>` or `o<n>` handle.
fn is_handle(token: &str) -> bool {
    let mut characters = token.chars();
    let is_prefixed = matches!(characters.next(), Some('c' | 'o'));
    let digits = characters.as_str();
    is_prefixed && !digits.is_empty() && digits.chars().all(|character| character.is_ascii_digit())
}

/// The decimal number `text` starts with.
fn leading_number(text: &str) -> Option<u32> {
    let digits: String = text.chars().take_while(char::is_ascii_digit).collect();
    digits.parse().ok()
}

/// A JSON rank vector of `dimensions` entries: `first`, then zeros.
pub(super) fn ranks(first: i64, dimensions: u32) -> String {
    let rest = (1..dimensions).map(|_| ",0").collect::<String>();
    format!("[{first}{rest}]")
}
