#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum FailurePoint {
    None,
    Collision,
    Stage,
    Write,
    Permission,
    PayloadSync,
    Rename,
    ParentSync,
}

#[derive(Clone, Debug)]
struct FakePublicationIo {
    failure: FailurePoint,
    stage_exists: bool,
    cleanup_succeeds: bool,
    parent_sync_supported: bool,
    calls: Vec<&'static str>,
}

impl FakePublicationIo {
    fn with_failure(failure: FailurePoint) -> Self {
        Self {
            failure,
            stage_exists: false,
            cleanup_succeeds: true,
            parent_sync_supported: true,
            calls: Vec::new(),
        }
    }
}

impl durable_file_publication::shell::PublicationIo for FakePublicationIo {
    fn supports_no_replace(&self) -> bool {
        true
    }

    fn supports_parent_sync(&self) -> bool {
        self.parent_sync_supported
    }

    fn create_stage(
        &mut self,
        _leaf: &str,
        _private_mode: u32,
    ) -> Result<
        durable_file_publication::shell::StageCreateDisposition,
        durable_file_publication::shell::MechanicalIoFailure,
    > {
        self.calls.push("stage");
        if self.failure == FailurePoint::Stage {
            return Err(durable_file_publication::shell::MechanicalIoFailure);
        }
        if self.failure == FailurePoint::Collision {
            return Ok(durable_file_publication::shell::StageCreateDisposition::AlreadyExists);
        }
        self.stage_exists = true;
        Ok(durable_file_publication::shell::StageCreateDisposition::Created)
    }

    fn write_all(
        &mut self,
        _bytes: &[u8],
    ) -> Result<(), durable_file_publication::shell::MechanicalIoFailure> {
        self.calls.push("write");
        (self.failure != FailurePoint::Write)
            .then_some(())
            .ok_or(durable_file_publication::shell::MechanicalIoFailure)
    }

    fn apply_permissions(
        &mut self,
        _mode: u32,
    ) -> Result<(), durable_file_publication::shell::MechanicalIoFailure> {
        self.calls.push("permission");
        (self.failure != FailurePoint::Permission)
            .then_some(())
            .ok_or(durable_file_publication::shell::MechanicalIoFailure)
    }

    fn sync_payload(&mut self) -> Result<(), durable_file_publication::shell::MechanicalIoFailure> {
        self.calls.push("payload-sync");
        (self.failure != FailurePoint::PayloadSync)
            .then_some(())
            .ok_or(durable_file_publication::shell::MechanicalIoFailure)
    }

    fn rename_stage(
        &mut self,
        _stage_leaf: &str,
        _destination_leaf: &str,
        _replacement: durable_file_publication::core::ReplacementMode,
    ) -> Result<
        durable_file_publication::shell::RenameDisposition,
        durable_file_publication::shell::MechanicalIoFailure,
    > {
        self.calls.push("rename");
        if self.failure == FailurePoint::Rename {
            return Err(durable_file_publication::shell::MechanicalIoFailure);
        }
        self.stage_exists = false;
        Ok(durable_file_publication::shell::RenameDisposition::Committed)
    }

    fn sync_parent(&mut self) -> Result<(), durable_file_publication::shell::MechanicalIoFailure> {
        self.calls.push("parent-sync");
        (self.failure != FailurePoint::ParentSync)
            .then_some(())
            .ok_or(durable_file_publication::shell::MechanicalIoFailure)
    }

    fn cleanup_stage(
        &mut self,
        _stage_leaf: &str,
    ) -> Result<(), durable_file_publication::shell::MechanicalIoFailure> {
        self.calls.push("cleanup");
        if self.cleanup_succeeds {
            self.stage_exists = false;
            Ok(())
        } else {
            Err(durable_file_publication::shell::MechanicalIoFailure)
        }
    }
}
