struct HostFileService;
struct HostProcessService;

impl HostFileService {
    fn read_labelled_artifact(&self, _label: &str) -> Result<String, &'static str> {
        Ok(String::from("safe summary"))
    }
}

impl HostProcessService {
    fn plan_command(&self, _label: &str) -> Result<(), &'static str> {
        Ok(())
    }
}

fn host_owned_shell_uses_injected_services(files: &HostFileService, processes: &HostProcessService) -> Result<(), &'static str> {
    let _summary = files.read_labelled_artifact("bounded-safe-label")?;
    processes.plan_command("reviewed-tool")?;
    Ok(())
}
