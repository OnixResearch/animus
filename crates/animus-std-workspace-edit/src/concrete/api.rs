/// Thin capability-relative workspace edit shell over caller-owned services.
pub struct WorkspaceEditHost<'a, S, N> {
    root: &'a cap_std::fs::Dir,
    services: &'a mut S,
    names: &'a mut N,
}

impl<'a, S: crate::concrete::HostServices, N: crate::concrete::StageNameService>
    WorkspaceEditHost<'a, S, N>
{
    /// Bind one caller-opened capability root and explicit host services.
    #[must_use]
    pub fn new(root: &'a cap_std::fs::Dir, services: &'a mut S, names: &'a mut N) -> Self {
        Self {
            root,
            services,
            names,
        }
    }

    /// Apply one request through the reviewed capability-relative publication mechanism.
    ///
    /// # Errors
    ///
    /// Fails closed on any gate, path, source, visibility, freshness, lock,
    /// publication, durability, release, or receipt issue. Failure after rename
    /// never claims rollback or an unchanged target.
    // r[impl agent_harness_runtime.hashed_edit_host.boundary]
    // r[impl agent_harness_runtime.hashed_edit_host.freshness]
    // r[impl agent_harness_runtime.hashed_edit_host.publication]
    pub fn apply(
        &mut self,
        request: &crate::concrete::HostRequest<'_>,
    ) -> Result<crate::evidence::EditReceipt, crate::concrete::HostError> {
        let root = self.root;
        let names = &mut *self.names;
        crate::concrete::flow::apply_with_publisher(
            root,
            self.services,
            request,
            |admitted, bytes| {
                crate::concrete::publication::publish_capability_relative(
                    root,
                    names,
                    crate::concrete::publication::MechanismInput {
                        admitted,
                        request,
                        bytes,
                    },
                )
            },
        )
    }

    /// Apply one request through an injected publication I/O seam.
    ///
    /// This method exists for deterministic mechanism and failure fixtures. The
    /// caller must keep the supplied I/O relative to the same opened parent.
    ///
    /// # Errors
    ///
    /// Returns the same bounded errors as [`Self::apply`].
    pub fn apply_with_publication_io(
        &mut self,
        io: &mut impl durable_file_publication::shell::PublicationIo,
        request: &crate::concrete::HostRequest<'_>,
    ) -> Result<crate::evidence::EditReceipt, crate::concrete::HostError> {
        let names = &mut *self.names;
        crate::concrete::flow::apply_with_publisher(
            self.root,
            self.services,
            request,
            |admitted, bytes| {
                crate::concrete::publication::publish_with_io(
                    io,
                    names,
                    crate::concrete::publication::MechanismInput {
                        admitted,
                        request,
                        bytes,
                    },
                )
            },
        )
    }
}
