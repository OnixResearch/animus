let schema = import "../component-catalog.ncl" in
let entries = [
  schema.add_dependencies ["missing.block"] (schema.make_component "runtime.block" "animus-runtime" schema.classes.runtime_prelude "runtime" schema.authorities.injected "runtime" ["cargo test -p animus-runtime --locked"] "animus_runtime::run_agent_loop"),
] in
{
  expected_issue = "COMPONENT_CATALOG_UNKNOWN_DEPENDENCY",
  component_lines = std.string.join "\n" (std.array.map schema.render_line entries),
  docs_markdown = "",
}
