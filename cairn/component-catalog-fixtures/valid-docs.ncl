let schema = import "../component-catalog.ncl" in
let entries = [
  schema.make_component "core.valid" "animus-core" schema.classes.reusable schema.features.always schema.authorities.none "core" ["cargo test -p animus-core --locked"] "animus_core::Intent",
] in
{
  expected_issue = "OK",
  component_lines = std.string.join "\n" (std.array.map schema.render_line entries),
  docs_markdown = schema.render_docs_fn entries,
}
