let schema = import "../component-catalog.ncl" in
let entries = [
  schema.make_component "owner.contract" "animus-runtime" schema.classes.reusable schema.features.always schema.authorities.none "runtime" ["cargo test -p animus-runtime --locked"] "animus_runtime::RuntimeConfig",
  schema.make_component "provider.host" "animus-std" schema.classes.shell "std" schema.authorities.shell "std-shell" ["cargo test -p animus-std --locked"] "animus_std::run",
  schema.make_component "std.live-steel-agent-shell" "animus-live-steel-agent" schema.classes.shell "live-provider" schema.authorities.shell "steel" ["cargo test -p animus-live-steel-agent --all-features --locked"] "animus_live_steel_agent::run_live_application",
] in
let ports = [
  schema.make_port "port.profile-drift" "owner.contract" ["provider.host"] ["std.live-steel-agent-shell"] "application" "exactly-one" "std.live-steel-agent-shell" "required" "live-provider" schema.authorities.shell "check-composition-activation-graph",
] in
{
  expected_issue = "COMPONENT_CATALOG_PROFILE_DRIFT",
  component_lines = std.string.join "\n" (
    std.array.map schema.render_line entries @ std.array.map schema.render_port_line ports
  ),
  docs_markdown = "",
}
