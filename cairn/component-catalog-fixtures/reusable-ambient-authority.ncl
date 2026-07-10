let schema = import "../component-catalog.ncl" in
let entries = [
  schema.make_component "core.ambient" "animus-core" schema.classes.reusable schema.features.always schema.authorities.shell "core" ["cargo test -p animus-core --locked"] "animus_core::Intent",
] in
{
  expected_issue = "COMPONENT_CATALOG_REUSABLE_AMBIENT_AUTHORITY",
  component_lines = std.string.join "\n" (std.array.map schema.render_line entries),
  docs_markdown = "",
}
