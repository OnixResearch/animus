let schema = import "../component-catalog.ncl" in
let entries = [
  schema.make_component "core.no-checks" "animus-core" schema.classes.reusable schema.features.always schema.authorities.none "core" [] "animus_core::Intent",
] in
{
  expected_issue = "COMPONENT_CATALOG_MISSING_CHECKS",
  component_lines = std.string.join "\n" (std.array.map schema.render_line entries),
  docs_markdown = "",
}
