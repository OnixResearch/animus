let schema = import "../component-catalog.ncl" in
let entries = [
  schema.make_component "component.one" "animus-core" schema.classes.reusable schema.features.always schema.authorities.none "core" ["cargo test -p animus-core --locked"] "animus_core::Intent",
  schema.make_component "component.two" "animus-adapters" schema.classes.reusable schema.features.always schema.authorities.none "adapter-contracts" ["cargo test -p animus-adapters --locked"] "animus_adapters::ModelRequest",
] in
let events = [
  schema.make_event "event.one" "component.one" ["component.two"] "ordered" "metadata-only" "component.one" "required" "acyclic" "check-composition-activation-graph",
  schema.make_event "event.two" "component.two" ["component.one"] "ordered" "metadata-only" "component.two" "required" "bounded" "check-composition-activation-graph",
] in
{
  expected_issue = "COMPONENT_CATALOG_UNSUPPORTED_EVENT_CYCLE",
  component_lines = std.string.join "\n" (
    std.array.map schema.render_line entries @ std.array.map schema.render_event_line events
  ),
  docs_markdown = "",
}
