let schema = import "../component-catalog.ncl" in
let entries = [
  schema.make_component "core.contract" "animus-core" schema.classes.reusable schema.features.always schema.authorities.none "core" ["cargo test -p animus-core --locked"] "animus_core::Intent",
  schema.make_component "host.provider" "animus-adapters" schema.classes.host_injection schema.features.always schema.authorities.injected "model" ["cargo test -p animus-adapters --locked"] "animus_adapters::ModelAdapter",
  schema.make_component "product.shell" "animus" schema.classes.shell "default" schema.authorities.shell "compatibility" ["cargo check -p animus --locked"] "animus",
] in
let ports = [
  schema.make_port "port.model" "core.contract" ["host.provider"] ["product.shell"] "application" "exactly-one" "product.shell" "required" "std" schema.authorities.shell "check-composition-activation-graph",
] in
{
  expected_issue = "COMPONENT_CATALOG_PORT_AUTHORITY_MISMATCH",
  component_lines = std.string.join "\n" (
    std.array.map schema.render_line entries @ std.array.map schema.render_port_line ports
  ),
  docs_markdown = "",
}
