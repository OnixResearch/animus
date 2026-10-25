//! Publishing admitted advice through Campaign's control session, one durable event at a time.

/// The host's durable control store: it admits one planned publication's history event and
/// compare-and-sets the control branch, then reports what it observed.
///
/// The manager never writes storage itself; this port is the only place a turn's advice becomes
/// durable.
pub trait DurableControlStore {
    /// Error of a failed or unobserved write.
    type Error;

    /// Makes `publication` durable and returns the host's evidence of it.
    ///
    /// # Errors
    ///
    /// Returns [`Self::Error`] when the store could not perform or observe the write.
    fn make_durable(
        &mut self,
        publication: &campaign_choregraph::ControlPublication,
    ) -> Result<campaign_choregraph::DurablePublicationEvidence, Self::Error>;
}

/// Why publication stopped before every event was committed.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PublicationCause<E> {
    /// The durable store failed.
    Store(E),
    /// The control session refused to plan or commit the event.
    Session(campaign_choregraph::AdapterError),
}

/// A publication that stopped early, with the session advanced through every committed event.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicationFailure<E> {
    /// The session after the last committed event; the host keeps it.
    pub session: campaign_choregraph::ControlSession,
    /// How many of the events were committed before the failure.
    pub committed: u64,
    /// Why publication stopped.
    pub cause: PublicationCause<E>,
}

/// Publishes `events` in order through `session`: plans each event's publication on the current
/// session, has `store` make it durable, and commits it on the store's evidence before planning
/// the next.
///
/// A failure keeps every event committed before it: the host holds the returned session, whose
/// remaining events still apply to it, and can publish them later or re-brief the advisor.
///
/// # Errors
///
/// Returns the boxed [`PublicationFailure`], holding the advanced session, when the session refuses
/// to plan or commit an event, or the store fails.
// r[impl agent_harness_runtime.campaign_manager.publication]
pub fn publish_events<S: DurableControlStore>(
    mut session: campaign_choregraph::ControlSession,
    events: &[campaign_core::CampaignControlEvent],
    store: &mut S,
) -> Result<campaign_choregraph::ControlSession, alloc::boxed::Box<PublicationFailure<S::Error>>> {
    let mut committed: u64 = 0;
    for event in events {
        let publication = match session.plan_publication(event.clone()) {
            Ok(publication) => publication,
            Err(error) => {
                return Err(failure(
                    session,
                    committed,
                    PublicationCause::Session(error),
                ));
            }
        };
        let evidence = match store.make_durable(&publication) {
            Ok(evidence) => evidence,
            Err(error) => return Err(failure(session, committed, PublicationCause::Store(error))),
        };
        session = match session.commit(&publication, &evidence) {
            Ok(advanced) => advanced,
            Err(refusal) => {
                let (unchanged, error) = refusal.into_parts();
                return Err(failure(
                    unchanged,
                    committed,
                    PublicationCause::Session(error),
                ));
            }
        };
        committed = committed.saturating_add(1);
    }
    debug_assert_eq!(u64::try_from(events.len()).ok(), Some(committed));
    debug_assert!(session.state().control_event_count() >= committed);
    Ok(session)
}

/// The failure of a publication stopped by `cause` after `committed` events.
fn failure<E>(
    session: campaign_choregraph::ControlSession,
    committed: u64,
    cause: PublicationCause<E>,
) -> alloc::boxed::Box<PublicationFailure<E>> {
    alloc::boxed::Box::new(PublicationFailure {
        session,
        committed,
        cause,
    })
}
