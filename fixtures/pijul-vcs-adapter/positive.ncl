let profile_value = (import "../../config/pijul-vcs-adapter.ncl").pijul_vcs_adapter in
{
  profile = profile_value,
  cases = [
    {
      name = "bounded-hash-log-query",
      operation = 'log,
      argv = ["log", "--repository", "/explicit/repository", "--channel", "main", "--hash-only", "--output-format", "json", "--limit", "128"],
      output = ["QRYOAJLYA5KO7JAYU6LMYHPU3NSZCTUMYEPCK3HNZIRIDZDWSMBAC"],
      expected_records = 1,
    },
    {
      name = "bounded-diff-query",
      operation = 'diff,
      argv = ["diff", "--repository", "/explicit/repository", "--channel", "main", "--json"],
      output = {"src/lib.rs" = [{operation = "edit", line = 1}]},
      expected_records = 1,
    },
    {
      name = "reviewed-channel-lines",
      operation = 'channel_list,
      output = "* main\n",
      expected_records = 1,
    },
  ],
}
