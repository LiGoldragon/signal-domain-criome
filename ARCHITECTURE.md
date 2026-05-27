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

## Pending schema-engine upgrade

**Status:** scheduled for migration to schema-language-based contract per `reports/designer/326-v13-spirit-complete-schema-vision.md` + `reports/designer/324-migration-mvp-spirit-handover-re-specification.md`.

**Target:** this contract's hand-written `signal_channel!` invocation converts to a single `domain-criome/domain-criome.schema` file (shared with the `domain-criome` daemon's repository). The brilliant macro library (`primary-ezqx.1`) reads the schema + emits this crate's wire types + ShortHeader projection + dispatcher binding + VersionProjection impls.

**Sequence:** per `primary-kbmi.2`. Spirit is the MVP pilot landing first via `primary-ezqx.1`; this contract's schema cutover after cloud.

**Per-component concerns:** Per `primary-kbmi.2`; schema cutover after cloud. The ordinary contract is paired with `owner-signal-domain-criome`; both legs of the policy-vs-working split appear in the shared `domain-criome.schema` file per the schema-language's separation discipline.

**References:**
- `reports/designer/326-v13-spirit-complete-schema-vision.md` — uniform header form + schema-language design
- `reports/designer/324-migration-mvp-spirit-handover-re-specification.md` — migration MVP + handover state
- `reports/designer/322-spirit-mvp-positional-schema-worked-example.md` — Spirit MVP worked example
- `reports/operator/174-schema-import-header-design-critique-2026-05-24.md` — header/body/feature separation + lowering rules
