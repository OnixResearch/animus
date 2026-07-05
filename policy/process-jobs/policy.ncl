let ResourceLimits = {
  timeout_seconds | Number,
  memory_bytes | Number,
  cpu_millis | Number,
  log_bytes | Number,
} in
let NotificationPolicy = {
  notify_on_complete | Bool,
} in
let MetadataField = {
  key | String,
  value | String,
} in
let EnvBinding = {
  key | String,
  value | String,
} in
let ProcessJobProfile = {
  name | String,
  backend | String,
  command | String,
  program | String,
  args | Array String,
  cwd | String,
  writable_paths | Array String,
  env | Array EnvBinding,
  resource_limits | ResourceLimits,
  notification | NotificationPolicy,
  metadata | Array MetadataField,
} in
let ProcessJobProfileManifest = {
  schema_version | Number,
  profiles | Array ProcessJobProfile,
} in
let ProcessJobProfilePolicy = {
  schema | String,
  profile_schema_version | Number,
  default_backend | String,
  allowed_backends | Array String,
  allowed_env_prefixes | Array String,
  allowed_cwd_prefixes | Array String,
  allowed_writable_path_prefixes | Array String,
  max_resource_limits | ResourceLimits,
  notification_defaults | NotificationPolicy,
  identity_metadata_keys | Array String,
  identity_hash_algorithm | String,
  identity_redaction | String,
} in
let process_job_schema = "animus.process-job-profile.v1" in
let policy_schema = "animus.process-job-profiles.policy.v1" in
let current_profile_schema_version = 1 in
let max_timeout_seconds = 600 in
let max_memory_bytes = 268435456 in
let max_cpu_millis = 1000 in
let max_log_bytes = 2097152 in
{
  process_job_profile_policy | ProcessJobProfilePolicy = {
    schema = policy_schema,
    profile_schema_version = current_profile_schema_version,
    default_backend = "native",
    allowed_backends = ["native", "pueue"],
    allowed_env_prefixes = ["APP_"],
    allowed_cwd_prefixes = ["/repo"],
    allowed_writable_path_prefixes = ["/repo/target"],
    max_resource_limits = {
      timeout_seconds = max_timeout_seconds,
      memory_bytes = max_memory_bytes,
      cpu_millis = max_cpu_millis,
      log_bytes = max_log_bytes,
    },
    notification_defaults = {
      notify_on_complete = true,
    },
    identity_metadata_keys = ["identity.purpose"],
    identity_hash_algorithm = "BLAKE3",
    identity_redaction = "hash-only",
  },

  sample_profile_manifest | ProcessJobProfileManifest = {
    schema_version = current_profile_schema_version,
    profiles = [
      {
        name = "quick-check",
        backend = "pueue",
        command = "",
        program = "cargo",
        args = ["check", "--tests"],
        cwd = "/repo",
        writable_paths = ["/repo/target"],
        env = [
          {
            key = "APP_MODE",
            value = "ci",
          },
        ],
        resource_limits = {
          timeout_seconds = 300,
          memory_bytes = 134217728,
          cpu_millis = 500,
          log_bytes = 1048576,
        },
        notification = {
          notify_on_complete = true,
        },
        metadata = [
          {
            key = "identity.purpose",
            value = "developer-smoke",
          },
        ],
      },
    ],
  },

  invalid_schema_fixture | ProcessJobProfileManifest = {
    schema_version = "not-a-number",
    profiles = [],
  },

  schema_label = process_job_schema,
}
