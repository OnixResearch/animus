{
  schema = "animus.terminal-image-negative-fixtures.v1",
  cases = [
    { name = "unsafe-handle", handle = "/private/image.png", expected = "unsafe_handle" },
    { name = "malformed-digest", digest = "abcd", expected = "malformed_digest" },
    { name = "oversized-image", byte_count = 8388609, expected = "image_bytes_exceeded" },
    { name = "invalid-source", source_width = 0, expected = "invalid_metadata" },
    { name = "oversized-cells", width = 257, expected = "terminal_geometry_exceeded" },
    { name = "missing-kitty-placement", place = false, expected = "text_fallback" },
    { name = "denied-fallback", allow_text_fallback = false, expected = "unsupported_operation" },
    { name = "stale-image-id", prior_image_id = "../old", expected = "invalid_metadata" },
    { name = "partial-write", bytes_written = 5, expected = "cleanup_required" },
    { name = "cleanup-failure", cleanup = "failed", expected = "cleanup_incomplete" },
    { name = "receipt-escape-leak", metadata = "bytes=\\x1b_G", expected = "evidence_leak" },
    { name = "receipt-path-leak", metadata = "/home/user/image.png", expected = "evidence_leak" },
  ],
}
