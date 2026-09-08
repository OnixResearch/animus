# Tasks: accept-responses-reasoning-text

## Implementation

- [x] [serial] [task:events] Route the reasoning text delta and done events through the existing reasoning paths with redacted frames and index correlation. r[agent_harness_runtime.responses_sse_framing.reasoning_text]
- [x] [serial] [task:parts] Select content part item kind from the part type and keep unknown-item correlation rejections. r[agent_harness_runtime.responses_sse_framing.reasoning_text.parts]
- [x] [serial] [task:release] Add positive and negative tests, replay the retained failing capture offline, run owning gates, publish, adopt the product pin, and close the lifecycle without deployment or activation. r[agent_harness_runtime.responses_sse_framing.reasoning_text.parts]
