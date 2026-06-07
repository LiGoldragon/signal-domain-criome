# signal-domain-criome Architecture

`signal-domain-criome` is the ordinary Signal contract for the
`domain-criome` component. It exposes intelligent Criome-domain resolution and
provider-neutral projection of desired domain state.

## Boundary

The contract describes Criome-domain meaning. It does not describe how
Cloudflare, Google, Hetzner, or any other provider applies that meaning.
Provider-specific execution belongs to `cloud`.

## Public Operations

- `Observe(Observation)` reads registered domains, delegations, or projected
  desired state.
- `Resolve(ResolutionQuery)` resolves a name in a named resolution scope.
- `Project(ProjectionQuery)` projects provider-neutral records and redirects.

## Owns

- Domain names, root names, and branch delegations, including the authority
  target a caller should follow for delegated names.
- Resolution queries and results.
- Provider-neutral domain-name-system record projections.
- Provider-neutral redirect projections.

## Does Not Own

- External provider credentials.
- Provider account identifiers.
- Provider API plans.
- Name-server process implementation.
- Meta registry mutation.

## Constraints

- Depend on `signal-frame`, not deprecated `signal-core`.
- Keep provider names out of the contract.
- Keep public operation roots as domain verbs, not Sema classes.

## Schema-emission status

**Status:** partial. `schema/lib.schema` is present and `build.rs` runs
`schema-rust-next`'s `GenerationPlan::wire_contract`, emitting the
checked-in witness at `src/schema/lib.rs`.

The crate's public top-level API still comes from the hand-written
`src/lib.rs` types and `signal_channel!` invocation. The remaining schema
cutover is to replace that duplicate hand-written surface with generated
re-exports from `src/schema/lib.rs`, then update downstream imports and tests.
The generated schema currently carries ordinary Signal wire vocabulary:
`Observe`, `Resolve`, `Project`, validation messages, observations,
resolutions, projections, typed rejections, and provider-neutral payload
records.

Nexus decisions, SEMA state, registry tables, projection runtime, and daemon
storage schemas live in `domain-criome`, not here. Meta registry and
projection-policy mutation messages live in `meta-signal-domain-criome`, not in
this ordinary contract.
