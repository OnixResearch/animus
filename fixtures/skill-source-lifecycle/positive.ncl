let policy = import "../../config/skill-source-lifecycle-policy.ncl" in
{
  frozen_exact = policy.modes == ["frozen", "tracked-candidate"],
  candidate_only = policy.automatic_activation == false,
  ambient_denied = policy.ambient_discovery == false,
  links_denied = policy.allow_symlinks == false && policy.allow_submodules == false,
  bounds_positive = policy.bounds.file_count_max > 0 && policy.bounds.total_bytes_max > 0,
}
