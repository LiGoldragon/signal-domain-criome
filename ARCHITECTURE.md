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
  When another domain daemon is authoritative, resolution returns
  `NotAuthoritative(AuthorityDelegation)` instead of rejecting the request.
  When this daemon is authoritative but has no address records for the name,
  resolution returns `NoRecords(NoRecords)`.
- `Project(ProjectionQuery)` projects provider-neutral records and redirects.

## Owns

- Domain names, root names, and branch delegations.
- Resolution queries, results, and authority delegation replies.
- `AuthorityDelegation` is the canonical redirect payload: branch
  `Delegation` describes registry shape, while authority delegation carries
  the endpoint a caller can ask next.
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
