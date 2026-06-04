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
- Owner-only registry mutation.

## Constraints

- Depend on `signal-frame`, not deprecated `signal-core`.
- Keep provider names out of the contract.
- Keep public operation roots as domain verbs, not Sema classes.

## Schema-engine upgrade track

When this contract moves from `signal_channel!` to schema-derived generation,
its schema lives in this repository and carries only ordinary Signal wire
vocabulary:

- `Input` roots for `Observe`, `Resolve`, `Project`, and validation messages.
- `Output` roots for observations, resolutions, projections, validations, and
  typed rejections.
- Domain, delegation, projection, and provider-neutral record payload types that
  cross the ordinary Signal wire.

Nexus decisions, SEMA state, registry tables, projection runtime, and daemon
storage schemas live in `domain-criome`, not here. Owner-only registry and
projection-policy mutation messages live in `meta-signal-domain-criome`, not in
this ordinary contract.
