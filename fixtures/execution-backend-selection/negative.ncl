let unavailable = false in
let available = true in
{
  schema = "animus.execution-backend-selection-negative-fixtures.v1",
  cases = [
    { name = "unknown-requirement", requirement = "unknown", expected = "unknown_requirement" },
    { name = "ambiguous", candidates = ['EmbeddedComponent, 'EmbeddedComponent], expected = "ambiguous_candidate" },
    { name = "unavailable", candidate_available = unavailable, broader_available = available, expected = "selected_backend_unavailable" },
    { name = "authority-drift", current_identity = "drifted", expected = "authority_drift" },
    { name = "resource-denial", resources_admitted = false, expected = "resource_denied" },
    { name = "silent-fallback", selected_available = unavailable, broader_available = available, expected = "selected_backend_unavailable" },
    { name = "sandbox-overclaim", evidence = "proves sandbox containment", expected = "unsafe_evidence" },
  ],
}
