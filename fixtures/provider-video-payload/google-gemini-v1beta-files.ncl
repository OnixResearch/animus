{
  schema = "animus.provider-video-native-fixture.v1",
  provider_profile = "google-gemini-v1beta-files-fixture",
  request_shape = {
    contents = [
      {
        role = "user",
        parts = [
          { text = "inspect" },
          {
            fileData = {
              mimeType = "video/mp4",
              fileUriHandle = "provider-file-video-redacted",
            },
          },
          { text = "summarize" },
        ],
      },
    ],
  },
  selected_safe_handle_forms = ["provider_file_id", "approved_remote_handle"],
  rejected_forms = ["raw_bytes", "base64", "data_url", "private_path", "credential", "upload_token_body"],
  non_claims = [
    "not-live-api-validation",
    "not-upload-completion",
    "not-provider-acceptance",
    "not-current-provider-limit-proof",
  ],
}
