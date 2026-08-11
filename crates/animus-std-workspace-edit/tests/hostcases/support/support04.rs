struct Fixture {
    temporary: tempfile::TempDir,
    root: cap_std::fs::Dir,
    resource_identity: String,
    base: animus_adapters::tool::hashed::Snapshot,
}

impl Fixture {
    fn new() -> Result<Self, String> {
        let temporary =
            tempfile::tempdir().map_err(|error| format!("temporary root: {error:?}"))?;
        std::fs::create_dir(temporary.path().join("nested"))
            .map_err(|error| format!("nested parent: {error:?}"))?;
        std::fs::write(temporary.path().join(RESOURCE), SOURCE)
            .map_err(|error| format!("fixture source: {error:?}"))?;
        let root =
            cap_std::fs::Dir::open_ambient_dir(temporary.path(), cap_std::ambient_authority())
                .map_err(|error| format!("capability root: {error:?}"))?;
        let resource_identity = animus_std_workspace_edit::logical_resource_identity(
            ROOT_ID,
            std::path::Path::new(RESOURCE),
        )
        .map_err(|error| format!("resource identity: {error:?}"))?;
        let base = base_snapshot(&resource_identity, SOURCE)?;
        Ok(Self {
            temporary,
            root,
            resource_identity,
            base,
        })
    }

    fn resource_path(&self) -> std::path::PathBuf {
        self.temporary.path().join(RESOURCE)
    }
}

fn apply_delete(
    fixture: &Fixture,
    services: &mut FakeServices,
    names: &mut Names,
    io: &mut FakePublicationIo,
) -> Result<animus_std_workspace_edit::EditReceipt, animus_std_workspace_edit::HostError> {
    assert_eq!(fixture.base.resource_identity(), fixture.resource_identity);
    assert!(!fixture.resource_identity.is_empty());
    let spans = [animus_std_workspace_edit::VisibleSpanDraft {
        start_tag: TAG_FIRST,
        end_tag: TAG_THIRD,
    }];
    let operations = [animus_std_workspace_edit::EditOperationDraft::Delete {
        start_tag: TAG_SECOND,
        end_tag: TAG_SECOND,
    }];
    let request = build_request(RequestInput {
        base: &fixture.base,
        resource_identity: &fixture.resource_identity,
        logical_resource: std::path::Path::new(RESOURCE),
        visible_spans: &spans,
        visible_boundaries: &[],
        operations: &operations,
    });
    animus_std_workspace_edit::WorkspaceEditHost::new(&fixture.root, services, names)
        .apply_with_publication_io(io, &request)
}
