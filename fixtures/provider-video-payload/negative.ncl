{
  schema = "animus.provider-video-payload-negative-fixtures.v1",
  cases = [
    { name = "wrong-modality", media_type = "image/png", expected = "unsupported_video_media_type" },
    { name = "zero-duration", duration_ms = 0, expected = "invalid_video_duration" },
    { name = "excess-frame-rate", frame_rate_millihz = 120001, expected = "invalid_video_frame_rate" },
    { name = "duplicate-identity", content_hashes = ["same", "same"], expected = "duplicate_video_identity" },
    { name = "count-overflow", video_count = 5, expected = "video_count_budget_exceeded" },
    { name = "missing-service", service = 'Missing, expected = "missing_video_service" },
    { name = "expired-handle", expired = true, expected = "expired_video_handle" },
    { name = "generic-conversion", mode = 'GenericJson, expected = "lossy_generic_conversion" },
    { name = "implicit-downgrade", image_support = true, video_support = false, expected = "unsupported_video" },
    { name = "receipt-leak", metadata = "provider_payload={body}", expected = "unsafe_descriptor" },
  ],
}
