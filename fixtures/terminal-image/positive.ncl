let policy = (import "../../config/terminal-image-policy.ncl").terminal_image_policy in
{
  schema = "animus.terminal-image-positive-fixtures.v1",
  rats_revision = policy.rats_revision,
  cases = [
    {
      name = "kitty-static-lifecycle",
      protocol = "kitty",
      operations = ["query", "transmit", "place"],
      cleanup_required = true,
      expected = "admitted",
    },
    {
      name = "iterm2-exact-row",
      protocol = "iterm2",
      operations = ["transmit"],
      expected = "admitted",
    },
    {
      name = "sixel-exact-row",
      protocol = "sixel",
      operations = ["transmit"],
      expected = "admitted",
    },
    {
      name = "metadata-only-receipt",
      redaction_profile = policy.receipt_redaction_profile,
      content_identity = "blake3-lowercase-hex",
      expected = "safe",
    },
  ],
}
