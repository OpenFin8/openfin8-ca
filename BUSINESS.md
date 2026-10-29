# OpenFin8:CA - Business Requirements & Functionality

> Audience: product owners, business analysts, and anyone evaluating what
> the Canadian adapter **does for the business**. For architecture read
> `README.md`; for wire-level detail read `DESIGN.md`.

## 1. Business Purpose

OpenFin8:CA is **Canadian open banking**: the FDX v6.x wire serving
Canada's consumer-driven-banking market. It is the service a Canadian
bank enables to become a data provider, and the surface a fintech
consumes as a data recipient. It delivers:

- **A compliant provider surface** - consented accounts, transactions,
  statements, customers, payment-network info, and money-movement
  initiation over `/fdx/v6/*`, with consent enforced on every request.
- **The recipient plane** - discovery, connection journeys, delegated
  reads - so the bank's fintech partners integrate without bespoke
  agreements.
- **Independent version cadence** - `OPENFIN8_CA_VERSION` (default
  6.5): the Canadian deployment upgrades on Canada's schedule, never
  coupling to another market's release.
- **Certification evidence** - the counters over audited traffic that
  Canadian FDX certification review consumes.

## 2. Business Scope (What the Service Provides)

| # | Capability area | What the business gets |
|---|---|---|
| 1 | Provider surface | Accounts + detail, transactions, statements (download via signed URL), customers, payment networks + revocation |
| 2 | Consent surfaces | Introspection, revocation (PUT + POST with reason + initiator), lifecycle duration windows |
| 3 | Money movement | Transfers (create/read/update/cancel), payments, payees, recurring payments, PIP (payment-initiation parties + methods) |
| 4 | Notifications | HMAC-signed webhook subscriptions + polling feed |
| 5 | Discovery | Capability, availability, certification metrics, operation-support statements |
| 6 | Recipient surfaces | `/v1/recipient/*`: provider discovery, consent start (DCR + PAR), connection list/detail, delegated account + transaction reads, refresh, revocation |
| 7 | Hosted consent UI | The RFW screens (scope list, account selection, approve/deny) the platform's consent host renders |
| 8 | Admin console | Provider profile, recipients, consents, CSDF, settings + the policy studio |
| 9 | Events | 16 outbound types (`openfin8:ca:*`) |
| 10 | Health | Fleet supervision surface |

**What the adapter does NOT own (business boundary):**

| Out of scope | Owned by |
|---|---|
| Consent ledger, registry, CSDF, notifications (the implementation) | openfin8-core (shared byte-for-byte with the US adapter) |
| Account/transaction data | bank8:Ledger (public API only) |
| Money-movement execution | bank8:Payments |
| The OAuth ceremony hosting | Sentin8:Auth |
| The US / UK surfaces | Their adapters (this service mounts only Canadian routes - profile isolation) |

## 3. Business Capabilities

### 3.1 The served surface

The complete FDX Core Exchange subset this deployment serves: 71 routes
across the provider surface, the recipient plane, the hosted consent
flows, the admin surfaces, and the internal step profile. Out-of-scope
FDX families (rewards, tax forms, investments) return the standard
unsupported response - never stubbed data.

### 3.2 The two-ledger ceremony

The fintech's connection journey: discover the bank (provider catalog),
start a consent (the bank's AS registers the fintech's client via DCR,
PAR carries the ceremony state), the customer authorizes on the hosted
screen, the callback activates the connection, and delegated reads
flow. The bank's grant ledger and the fintech's registry reconcile
through the event family - no cross-tenant access ever.

### 3.3 Canadian market posture

The Canadian profile keeps deliberate, isolated extension points for
the federal consumer-driven-banking framework as it finalizes:
directory/accreditation participation (a `registration_authority`
hook), bilingual (EN/FR) consent wording (the locale-aware screen
trait), and Canadian payment-rail identifiers (network ids are profile
data, not schema).

## 4. Business Events (Outbound)

Family `openfin8:ca:*`, grammar
`openfin8:ca:{resource}:{operation}`: consents
(created/updated/revoked), recipient consents
(started/refreshed/revoked), transfers, payments, recurring payments,
payees, PIP parties, payment-network revocations, notification
subscriptions, provider-profile updates.

## 5. Business Rules & Invariants

- No consent, no data - and no provider route bypasses the middleware.
- Revocation is immediate and terminal.
- Version selection is per-environment configuration
  (`OPENFIN8_CA_VERSION` / `sys.services.config`), not a redeploy.
- The FDX API specification is licensed (FDX API License Agreement,
  distributed exclusively by FDX) - **never published by this
  repository**; the published spec carries OpenFin8-owned routes only
  (CI drift-guarded); customers supply their licensed copy under
  gitignored `specs/`.
- This adapter mounts only Canadian routes (profile isolation - a
  Canadian deployment cannot serve another market's surface).

## 6. Target-User Roles & Use Cases

| Role | Use case |
|---|---|
| Canadian bank (provider) | Enable `openfin8:ca` in the bank's environment; serve the FDX surface; read consents/audit/metrics in the admin console |
| Fintech (recipient) | Discover providers, run consent journeys, consume consented data via the recipient surfaces or the public provider API |
| Bank customer | Authorizes on the hosted consent screen; reviews and revokes via the bank's app (ebank's consent dashboard) |
| Regulator / certification | Reviews the certification-metrics evidence derived from audited traffic |

## 7. KPIs / Success Criteria

- **Market readiness**: enabling the service is configuration - no
  engineering work beyond enrollment.
- **Certification readiness**: FDX certification metrics current at
  query time from audited traffic.
- **Spec policy compliance**: the drift guard proves zero licensed
  material in the published spec on every CI run.
- **Identical-code guarantee**: zero implementation drift from the US
  adapter (auditable in the repo: only Profile + registration differ).

## 8. Dependencies & Reliance on Other Services

| Dependency | What the adapter gets |
|---|---|
| openfin8-core | The entire implementation (consent ledger, registry, CSDF, wire, notifications, admin) |
| bank8:Ledger / bank8:Payments | The data plane - via public APIs + `bank8::` contracts (the OBVERT boundary) |
| Sentin8:Auth | The DPoP-ceremony profile, the grant-seed event |
| Sentin8:Bus / Secrets / Storage / Sys / Flow | The platform: events, token custody, statements, enablement, step execution |

## 9. Acceptance Criteria

- Build + tests green (publication guard included); CI green on the
  OpenFin8 org.
- The served surface identical to what the predecessor (bank8-fdx)
  served - under the Canadian identity (schema `openfin8_ca`, scopes
  `openfin8:ca:*`, events `openfin8:ca:*`, `OPENFIN8_CA_*` envs).
- ebank/efintech/admin and the e2e suites retargeted and green.
- The published spec (35 OpenFin8-owned routes) renders on
  openfin8.dev.

## 10. Open Business Questions

- Does the federal technical annex mandate a distinct API versioning
  scheme or namespace (the `provider_prefix` hook is where it lands)?
- When directory/accreditation participation is defined, does the
  DCR-direct path remain, or does the registry-authority hook become
  mandatory?
- Should bilingual consent wording land ahead of the federal
  requirement (a Canadian UX differentiator) or wait for the
  published wording rules?

---

**Architecture & Design:** openfin8:ca and its underlying concepts were
designed by [Hamid Jahanzad](https://www.linkedin.com/in/hamidjahanzad/).