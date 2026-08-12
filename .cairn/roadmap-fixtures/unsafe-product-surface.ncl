let Fixture = {
  expected_issue | String,
  roadmap_lines | String,
}
in
{
  expected_issue = "ROADMAP_UNSAFE_PRODUCT_SURFACE",
  roadmap_lines =
    std.string.join
      "\n"
      [
        "tier|ui-headless||UI and headless surfaces",
        "active|ui-surface",
        "change|ui-surface|product-surface|ui-headless||",
      ],
} | Fixture
