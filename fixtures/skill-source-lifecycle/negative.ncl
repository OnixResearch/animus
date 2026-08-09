let policy = import "../../config/skill-source-lifecycle-policy.ncl" in
{
  automatic_activation_rejected = policy.automatic_activation == true,
  ambient_discovery_rejected = policy.ambient_discovery == true,
  symlink_rejected = policy.allow_symlinks == true,
  unbounded_rejected = policy.bounds.file_count_max == 0,
}
