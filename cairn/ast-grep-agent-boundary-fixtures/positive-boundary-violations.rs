struct OpenAiClient;
struct AnthropicClient;
struct ProviderClient;

impl OpenAiClient {
    fn new(_token: &str) -> Self {
        Self
    }
}

impl AnthropicClient {
    fn new(_token: &str) -> Self {
        Self
    }
}

impl ProviderClient {
    fn new(_token: &str) -> Self {
        Self
    }
}

fn reusable_surface_with_ambient_effects(token: &str) {
    let _contents = std::fs::read_to_string("workspace/private.txt");
    let _child = std::process::Command::new("tool");
    let _client = OpenAiClient::new(token);
    let _provider = ProviderClient::new(token);
    unsafe {
        let _raw = 1 as *const i32;
        let _ = _raw;
    }
    panic!("candidate boundary finding");
}
