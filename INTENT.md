# INTENT — signal-domain-criome

*The ordinary peer-callable wire contract for the `domain-criome`
component. Companion to `ARCHITECTURE.md` and `Cargo.toml`.
Maintenance: `primary/skills/repo-intent.md`.*

## Repo-scope only

This file carries only the intent that is FOR this `signal-domain-criome`
contract. Workspace-shape intent stays in the primary workspace
`primary/INTENT.md`. Component daemon intent stays in
`domain-criome/INTENT.md`. Owner/meta policy intent stays in
`meta-signal-domain-criome/INTENT.md`.

## Why this repo exists

`signal-domain-criome` is the **ordinary peer-callable wire contract**
for the `domain-criome` component. It exposes intelligent Criome-domain
resolution and provider-neutral projection of desired domain state:
peers observe registered domains, delegations, and projected state;
resolve a name in a named resolution scope; and project provider-neutral
DNS records and redirects. Owner-only registry and projection-policy
mutation lives in `meta-signal-domain-criome`; registry tables,
projection runtime, Nexus decisions, SEMA state, and daemon storage live
in `domain-criome`.

## Meaning here, execution elsewhere

The contract describes Criome-domain *meaning*. It does **not** describe
how Cloudflare, Google, Hetzner, or any other provider *applies* that
meaning — provider-specific execution belongs to `cloud`. Provider names
stay out of this contract entirely.

## The channel shape

The ordinary domain-criome channel carries:

- **Requests:** `Observe(Observation)` reads registered domains,
  delegations, or projected desired state; `Resolve(ResolutionQuery)`
  resolves a name in a named resolution scope; `Project(ProjectionQuery)`
  projects provider-neutral records and redirects.
- **Replies:** observations, resolutions, projections, validations, and
  typed rejections.

## Channels are closed, boundaries are named

- Wire enums are closed. No `Unknown` escape hatch.
- Provider names are kept out of the contract; only provider-neutral
  domain meaning crosses this wire.
- Public operation roots are domain verbs, not Sema classes.
- A delegation names the authority target a caller should follow for
  delegated names.

## Wire vocabulary discipline

Per `primary/skills/contract-repo.md` §"Public contracts use
contract-local operation verbs":

- Operation roots are domain verbs in verb form: `Observe`, `Resolve`,
  `Project`.
- Payload records are domain nouns: `Observation`, `ResolutionQuery`,
  `ProjectionQuery`.

## Constraints

- This crate carries only typed wire vocabulary, NOTA codecs, and
  round-trip witnesses. No runtime code.
- Depend on `signal-frame`, not deprecated `signal-core`.
- Every operation and reply variant round-trips through both rkyv frames
  and NOTA text.
- When this contract moves from `signal_channel!` to schema-derived
  generation, its `schema/` carries only ordinary Signal wire
  vocabulary; Nexus and SEMA schemas live in `domain-criome`.

## Non-ownership

This crate does not own:

- external provider credentials, account identifiers, or provider API
  plans;
- name-server process implementation;
- owner-only registry mutation (that lives in
  `meta-signal-domain-criome`);
- provider-specific execution (that belongs to `cloud`).

## See also

- `ARCHITECTURE.md` — detailed channel shape, owned vocabulary, and the
  schema-engine upgrade track.
- `../domain-criome/INTENT.md` — daemon-side intent when it lands.
- `../meta-signal-domain-criome/INTENT.md` — owner meta-signal policy
  contract.
- `primary/skills/contract-repo.md` — contract repo discipline and
  naming rules.
- `primary/skills/component-triad.md` — repo triad structure and wire
  layers.
