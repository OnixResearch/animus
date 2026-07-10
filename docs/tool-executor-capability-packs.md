# Tool executor capability packs

Tool executor packs are host-owned descriptors for admitted tool authority. A pack binds trusted tool definition metadata, schema identity, executor service label, exact capability requirement, output policy, mutation policy, approval posture, and prompt-exposure state before prompt exposure or dispatch.

The reusable crates expose DTOs, admission logic, deterministic fake executors, and safe receipts. They do not ship product tools, load plugins, discover global catalogs, run shell commands, spawn processes, access filesystems, call networks, or grant ambient authority.

Receipts store safe labels, counts, BLAKE3 identities, output hashes, failure classes, and redaction markers only.
