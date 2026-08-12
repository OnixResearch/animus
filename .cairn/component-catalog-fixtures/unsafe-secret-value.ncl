let schema = import "../component-catalog.ncl" in
let entries = [
  schema.set_evidence ["secret-value=forbidden"] (schema.make_component "core.unsafe-secret-value" "animus-core" schema.classes.reusable schema.features.always schema.authorities.none "core" ["cargo test -p animus-core --locked"] "animus_core::Intent"),
] in
{
  expected_issue = "COMPONENT_CATALOG_UNSAFE_EVIDENCE_FIELD",
  component_lines = std.string.join "\n" (std.array.map schema.render_line entries),
  docs_markdown = "",
}
