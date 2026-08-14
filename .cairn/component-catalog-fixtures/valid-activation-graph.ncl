let schema = import "../component-catalog.ncl" in
let entries = [
  schema.make_component "core.contract" "animus-core" schema.classes.reusable schema.features.always schema.authorities.none "core" ["cargo test -p animus-core --locked"] "animus_core::Intent",
  schema.make_component "host.provider" "animus-std" schema.classes.shell "std" schema.authorities.shell "std-shell" ["cargo test -p animus-std --locked"] "animus_std::run_with_model",
  schema.make_component "product.shell" "animus" schema.classes.shell "default" schema.authorities.shell "compatibility" ["cargo check -p animus --locked"] "animus",
] in
let ports = [
  schema.make_port "port.model" "core.contract" ["host.provider"] ["product.shell"] "application" "exactly-one" "product.shell" "required" "std" schema.authorities.shell "check-composition-activation-graph",
] in
let events = [
  schema.make_event "event.model-result" "core.contract" ["product.shell"] "ordered" "metadata-only" "product.shell" "required" "acyclic" "check-composition-activation-graph",
] in
{
  expected_issue = "OK",
  component_lines = std.string.join "\n" (
    std.array.map schema.render_line entries
    @ std.array.map schema.render_port_line ports
    @ std.array.map schema.render_event_line events
  ),
  docs_markdown = schema.render_activation_docs_fn entries ports events,
}
