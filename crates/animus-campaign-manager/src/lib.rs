#![no_std]
#![forbid(unsafe_code)]
//! An AI campaign manager for Campaign exploration campaigns.
//!
//! The manager is an Animus coordination role. It steers one Campaign campaign through Campaign's
//! advisory seam: it reads a bounded brief of the campaign, asks an injected Animus model adapter
//! for advice, turns the model's reply into an advisory proposal, and has Campaign review it under
//! the host's mandate. Campaign admits the whole proposal as ordinary control events, or refuses
//! it; the host then publishes the events through Campaign's control session.
//!
//! # One turn
//!
//! 1. [`advise`] briefs the campaign (`campaign_core::brief_advisor`), renders the brief with
//!    [`render_brief`], calls the model once, parses its reply with [`parse_reply`], builds the
//!    proposal with [`build_proposal`], and reviews it with
//!    `campaign_core::review_advisory_proposal`.
//! 2. [`publish_events`] publishes the admitted events in order through a
//!    `campaign_choregraph::ControlSession`, making each durable through the host's
//!    [`DurableControlStore`] before committing it.
//!
//! # Authority
//!
//! The crate performs no effect of its own: the model call and every durable write go through
//! injected ports, and it reads no files, environment, clocks, randomness, or network. The model
//! sees only short handles, the host's labels, the campaign's counters, and the mandate's grants;
//! it never sees Campaign identities, so a reply can only act on what the brief presented, and
//! the mandate and Campaign's review bound everything it asks for. Admitted advice grants no
//! execution, acceptance, or release authority: the host still decides when to run a selection,
//! and only after that selection's own durable publication.
//!
//! The crate is `AGPL-3.0-or-later` because it links Campaign, and it stays outside the
//! workspace's default members.
// r[impl agent_harness_runtime.campaign_manager.boundary]

extern crate alloc;

mod advice;
mod answer;
mod drafting;
mod durability;
mod fault;
mod host;
mod prompt;

pub use advice::{ManagerTurn, advise};
pub use answer::{
    AddReply, ManagerReply, PruneReply, RetireReply, ScoreReply, StopReply, parse_reply,
};
pub use drafting::build_proposal;
pub use durability::{DurableControlStore, PublicationCause, PublicationFailure, publish_events};
pub use fault::{ManagerError, TurnFailure};
pub use host::{
    CandidateLabel, CandidateMenu, CandidateOffer, MAX_LABEL_CHARS, MAX_OBJECTIVE_CHARS,
    MAX_OFFERS, MAX_REASON_CHARS, ManagerSetup,
};
pub use prompt::{BriefHandles, RenderedBrief, render_brief};
