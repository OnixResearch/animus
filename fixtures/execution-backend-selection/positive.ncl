{
  schema = "animus.execution-backend-selection-fixtures.v1",
  cases = [
    { name = "embedded-component", candidates = ['EmbeddedComponent], expected = 'EmbeddedComponent },
    { name = "guarded-process", candidates = ['GuardedLocalProcess], expected = 'GuardedLocalProcess },
    { name = "isolated-host", candidates = ['IsolatedHost], expected = 'IsolatedHost },
    { name = "external-machine", candidates = ['ExternalMachine], expected = 'ExternalMachine },
  ],
}
