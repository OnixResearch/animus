let schema = import "../component-catalog.ncl" in
let entries = [
  schema.make_component "core.contract" "animus-core" schema.classes.reusable schema.features.always schema.authorities.none "core" ["cargo test -p animus-core --locked"] "animus_core::Intent",
  schema.make_component "product.shell" "animus" schema.classes.shell "default" schema.authorities.shell "compatibility" ["cargo check -p animus --locked"] "animus",
] in
let ports = [
  schema.make_port "port.model" "core.contract" [] ["product.shell"] "application" "exactly-one" "product.shell" "required" "std" schema.authorities.shell "check-composition-activation-graph",
] in
{
  expected_issue = "COMPONENT_CATALOG_ORPHAN_REQUIRED_PORT",
  component_lines = std.string.join "\n" (
    std.array.map schema.render_line entries @ std.array.map schema.render_port_line ports
  ),
  docs_markdown = "",
}
