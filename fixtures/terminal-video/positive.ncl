let policy = (import "../../config/terminal-video-policy.ncl").terminal_video_policy in
{
  schema = "animus.terminal-video-positive-fixtures.v1",
  rats_revision = policy.rats_revision,
  cases = [
    {
      name = "ordered-static-frames",
      backend = "static-replacement",
      frames = [0, 1],
      actions = ["display", "request", "display", "finish", "cleanup"],
      expected = "finished",
    },
    {
      name = "kitty-animation-exact-row",
      backend = "kitty-animation",
      frame_adapter = "rat-image-static-frame-transfer",
      lifecycle_adapter = "host-admitted-kitty-animation-controls",
      expected = "admitted",
    },
    {
      name = "bounded-backpressure",
      queue_frames = policy.max_queue_frames,
      action = "drop-before-retention",
      expected = "bounded",
    },
    {
      name = "metadata-only-receipt",
      redaction_profile = policy.receipt_redaction_profile,
      identity = "blake3-lowercase-hex",
      expected = "safe",
    },
  ],
}
