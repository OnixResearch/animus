let schema = import "../component-catalog.ncl" in
let entries = [
  schema.make_component "core.valid" "animus-core" schema.classes.reusable schema.features.always schema.authorities.none "core" ["cargo test -p animus-core --locked"] "animus_core::Intent",
  schema.add_dependencies ["core.valid"] (schema.make_component "adapter.port" "animus-adapters" schema.classes.host_injection schema.features.always schema.authorities.injected "model" ["cargo test -p animus-adapters model --locked"] "animus_adapters::ModelAdapter"),
  schema.add_dependencies ["adapter.port"] (schema.make_component "std.shell" "animus-std" schema.classes.shell "std" schema.authorities.shell "std-shell" ["cargo test -p animus-std --locked"] "animus_std::run_with_model"),
  schema.add_dependencies ["core.valid"] (schema.make_component "preserves.bridge" "animus-preserves" schema.classes.optional_bridge "preserves" schema.authorities.none "preserves" ["cargo test -p animus-preserves --locked"] "animus_preserves"),
] in
{
  expected_issue = "OK",
  component_lines = std.string.join "\n" (std.array.map schema.render_line entries),
  docs_markdown = "",
}
