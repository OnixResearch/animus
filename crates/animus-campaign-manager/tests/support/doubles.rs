//! Test doubles: a model that answers with scripted replies and an in-memory durable control
//! store.

/// A model that answers with scripted replies and records every prompt it was asked.
pub struct ScriptedModel {
    /// Replies still to give, first one next.
    pub replies: Vec<String>,
    /// Prompts received so far.
    pub prompts: Vec<String>,
}

impl ScriptedModel {
    /// A model that gives `reply` once.
    pub fn replying(reply: &str) -> Self {
        Self {
            replies: vec![String::from(reply)],
            prompts: Vec::new(),
        }
    }

    /// A model that fails every call.
    pub const fn silent() -> Self {
        Self {
            replies: Vec::new(),
            prompts: Vec::new(),
        }
    }
}

impl
    animus_adapters::ComponentAdapter<animus_adapters::ModelRequest, animus_adapters::ModelResponse>
    for ScriptedModel
{
    type Error = &'static str;

    fn call(
        &mut self,
        request: &animus_adapters::ModelRequest,
    ) -> Result<animus_adapters::ModelResponse, Self::Error> {
        self.prompts.push(String::from(request.prompt()));
        if self.replies.is_empty() {
            return Err("model unavailable");
        }
        let reply = self.replies.remove(0);
        animus_adapters::ModelResponse::new(&reply).map_err(|_| "invalid scripted reply")
    }
}

/// An in-memory durable store that observes every write as durable, optionally failing on one
/// write.
pub struct MemoryStore {
    /// Writes performed so far.
    pub writes: u64,
    /// The zero-based write that fails, if any.
    pub fail_at: Option<u64>,
}

impl MemoryStore {
    /// A store whose every write is durable.
    pub const fn durable() -> Self {
        Self {
            writes: 0,
            fail_at: None,
        }
    }

    /// A store whose zero-based write `write` fails.
    pub const fn failing_at(write: u64) -> Self {
        Self {
            writes: 0,
            fail_at: Some(write),
        }
    }
}

impl animus_campaign_manager::DurableControlStore for MemoryStore {
    type Error = &'static str;

    fn make_durable(
        &mut self,
        publication: &campaign_choregraph::ControlPublication,
    ) -> Result<campaign_choregraph::DurablePublicationEvidence, Self::Error> {
        if self.fail_at == Some(self.writes) {
            return Err("store unavailable");
        }
        self.writes = self.writes.saturating_add(1);
        let plan = publication.branch_plan();
        Ok(campaign_choregraph::DurablePublicationEvidence {
            history_event_id: publication.history_event_id(),
            branch_before: plan.before.clone(),
            branch_after: plan.after.clone(),
            durable_event: true,
            durable_branch_update: true,
        })
    }
}
