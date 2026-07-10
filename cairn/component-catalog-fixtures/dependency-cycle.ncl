let schema = import "../component-catalog.ncl" in
let entries = [
  schema.add_dependencies ["cycle.two"] (schema.make_component "cycle.one" "animus-core" schema.classes.reusable schema.features.always schema.authorities.none "core" ["cargo test -p animus-core --locked"] "animus_core::Intent"),
  schema.add_dependencies ["cycle.one"] (schema.make_component "cycle.two" "animus-adapters" schema.classes.reusable schema.features.always schema.authorities.none "adapter-contracts" ["cargo test -p animus-adapters --locked"] "animus_adapters::ModelRequest"),
] in
{
  expected_issue = "COMPONENT_CATALOG_DEPENDENCY_CYCLE",
  component_lines = std.string.join "\n" (std.array.map schema.render_line entries),
  docs_markdown = "",
}
