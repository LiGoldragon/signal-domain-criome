# signal-domain-criome — Agent Instructions

Read this repository's `INTENT.md`, `ARCHITECTURE.md`, and this file before
editing.

This repository is a pure Signal contract crate. It declares the ordinary
Criome-domain resolution and projection vocabulary. It contains no daemon,
storage, name-server runtime, provider clients, or meta policy.

Do not add Cloudflare, Google, Hetzner, or provider-specific fields here.
Provider execution belongs to `cloud` and `signal-cloud`.
