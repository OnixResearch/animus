let policy = (import "../../config/provider-video-payload-policy.ncl").provider_video_payload_policy in
{
  schema = "animus.provider-video-payload-positive-fixtures.v1",
  cases = [
    {
      name = "ordered-mp4-safe-handle",
      provider_profile = "google-gemini-v1beta-files-fixture",
      media_type = "video/mp4",
      safe_handle = "provider-file-video-redacted",
      byte_count = 10240,
      duration_ms = 12000,
      expected = "included",
    },
    {
      name = "metadata-only-receipt",
      redaction_profile = policy.receipt_redaction_profile,
      content_identity = "blake3-hex",
      expected = "safe",
    },
  ],
}
