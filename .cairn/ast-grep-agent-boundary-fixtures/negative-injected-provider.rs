trait ModelDispatchService {
    fn plan_model_dispatch(&self, provider_label: &str) -> Result<(), &'static str>;
}

fn runtime_uses_injected_provider(service: &dyn ModelDispatchService) -> Result<(), &'static str> {
    service.plan_model_dispatch("reviewed-provider")
}
