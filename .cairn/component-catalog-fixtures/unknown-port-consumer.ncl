let schema = import "../component-catalog.ncl" in
let entries = [
  schema.make_component "core.contract" "animus-core" schema.classes.reusable schema.features.always schema.authorities.none "core" ["cargo test -p animus-core --locked"] "animus_core::Intent",
  schema.make_component "host.provider" "animus-std" schema.classes.shell "std" schema.authorities.shell "std-shell" ["cargo test -p animus-std --locked"] "animus_std::run_with_model",
] in
let ports = [
  schema.make_port "port.model" "core.contract" ["host.provider"] ["unknown.consumer"] "application" "exactly-one" "host.provider" "required" "std" schema.authorities.shell "check-composition-activation-graph",
] in
{
  expected_issue = "COMPONENT_CATALOG_UNKNOWN_PORT_CONSUMER",
  component_lines = std.string.join "\n" (
    std.array.map schema.render_line entries @ std.array.map schema.render_port_line ports
  ),
  docs_markdown = "",
}
