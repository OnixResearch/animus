let profile = (import "../../config/trajectory-transcript-adapter.ncl").trajectory_transcript_adapter in
if profile.enabled_by_default == false
  && profile.cohort.version == "0.2.0"
  && std.array.length profile.cohort.sources == 7
  && profile.content.reasoning == "omit"
then { valid = true }
else std.contract.blame "TRAJECTORY_POSITIVE_FIXTURE"
