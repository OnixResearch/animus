let schema = import "../component-catalog.ncl" in
let entries = [
  schema.make_component "core.unknown-class" "animus-core" "ambient-contract" schema.features.always schema.authorities.none "core" ["cargo test -p animus-core --locked"] "animus_core::Intent",
] in
{
  expected_issue = "COMPONENT_CATALOG_UNKNOWN_CLASS",
  component_lines = std.string.join "\n" (std.array.map schema.render_line entries),
  docs_markdown = "",
}
