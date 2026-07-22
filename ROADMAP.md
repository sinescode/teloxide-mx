# ROADMAP: Beyond 100% aiogram Parity

**Product goal:** Make `teloxide_max` the default Rust Telegram bot framework — equal to aiogram in every practical dimension, superior in safety, performance, and ops.  
**Baseline:** Bot API **10.2** parity with aiogram **3.30.x** (~99.5%+ practical; see `GAP_ANALYSIS.md`).  
**Date:** 2026-07-22  

---

## North-star metrics

| Metric | Target |
|--------|--------|
| Bot API lag vs Telegram release | ≤ 7 days for critical methods/types |
| CI (PR) median duration | ≤ 12 minutes (stable-only matrix) |
| Docs: book pages for every public util | 100% |
| Migration success (aiogram users ship in <1 day) | Survey / examples |
| Zero `FIXME` in user-facing webhook/security paths | Maintain |

---

## Phase 0 — Ship confidence (now)

| ID | Item | Why | Effort |
|----|------|-----|--------|
| P0.1 | Green CI on `main` + PRs | Only verification gate on disk-constrained hosts | M |
| P0.2 | Release notes for webhook IP filter, allowed_updates rebind, event isolation | Users need upgrade clarity | S |
| P0.3 | Pin / document TBA version in root README | Avoid “which API?” confusion | S |

**Done when:** CI green; CHANGELOG entries published; `GAP_ANALYSIS.md` linked from README.

---

## Phase 1 — Developer experience (0–1 release)

| ID | Item | Notes |
|----|------|-------|
| P1.1 | Book: `content-type`, `magic-data`, `event-isolation`, webhook security | Link from `SUMMARY.md` |
| P1.2 | Example: scenes + event isolation | Mirror aiogram `quiz_scene.py` |
| P1.3 | Example: webhooks with `telegram_ip_filter()` | Production template |
| P1.4 | Prelude audit | Ensure new utils re-exported where discoverable |
| P1.5 | Optional `#[teloxide_max::handler]` macro research | Decorator-like DX without fighting the type system |
| P1.6 | Error messages | When DI fails, point to missing type / filter |

---

## Phase 2 — Performance & scale (1–2 releases)

| ID | Item | Notes |
|----|------|-------|
| P2.1 | `cargo nextest` in CI | Faster test scheduling |
| P2.2 | Throttle adaptor benchmarks | Document throughput vs aiogram |
| P2.3 | Optional sccache / shared GHA cache keys | Already partially done |
| P2.4 | Streaming download / large media paths | Audit multipart edge cases |
| P2.5 | Worker-pool tuning guide | `distribution_function`, queue sizes |

---

## Phase 3 — Bot API tracking process (continuous)

| ID | Item | Notes |
|----|------|-------|
| P3.1 | Codify TBA update runbook | schema check → codegen → field audit → e2e → release |
| P3.2 | Automate method-set diff vs aiogram | Script in CI (optional) |
| P3.3 | Track TBA 10.3+ as soon as published | Prefer correctness over same-day hype |
| P3.4 | Keep `rust-toolchain.toml` nightly pin in sync with CI | Single source of truth |

---

## Phase 4 — Ecosystem (ongoing)

| ID | Item | Notes |
|----|------|-------|
| P4.1 | Official metrics middleware (Prometheus / OTel) | Observability parity with mature stacks |
| P4.2 | Rate-limit / flood-wait recipes | Document + helpers on top of `Throttle` |
| P4.3 | Testing kit polish | Mock bot, update fixtures, golden JSON |
| P4.4 | Community storage plugins | If core stays batteries-included |
| P4.5 | WASM / edge constraints | Expand serverless docs |

---

## Phase 5 — Optional backends (nice-to-have)

| ID | Item | Priority |
|----|------|----------|
| P5.1 | Warp or Actix webhook listener | Low (axum is default) |
| P5.2 | Hyper-only minimal webhook | Low |
| P5.3 | Native `getUpdates` long-poll batching knobs | Medium if users request |

---

## Explicit non-goals

- Replicating Python decorator syntax in Rust  
- Shipping `unsafe` for micro-benchmarks  
- Local full `cargo` matrix on developer laptops with constrained disk (CI is the gate)  
- 1:1 module path cloning of aiogram (prefer idiomatic Rust names with migration aliases)

---

## Suggested release train

| Version | Theme |
|---------|--------|
| **0.17.x** | Parity polish: webhooks, isolation, ContentType, lifecycle hooks |
| **0.18** | DX book + examples + CI hardening |
| **0.19** | Performance + observability middleware |
| **1.0** | Stable API surface after ≥1 TBA cycle without breaking changes |

---

## Immediate next actions (checklist)

- [ ] Push branch / PR so optimized CI runs full gate  
- [ ] Confirm webhook + isolation tests in CI (unit tests in `event_isolation`, webhook security tests)  
- [ ] Update `teloxide_max/CHANGELOG.md` with user-facing notes  
- [ ] Link `GAP_ANALYSIS.md` + `ROADMAP.md` from root / crate README  
- [ ] After CI green: tag release candidate if cutting 0.17.x  

---

## References

- `GAP_ANALYSIS.md` — authoritative parity status  
- `01_FEATURE_PARITY_REPORT.md` … `07_DEVELOPER_EXPERIENCE_REPORT.md` — historical deep dives  
- `teloxide_max/MIGRATION_GUIDE.md` — aiogram migration  
- `teloxide_max/.github/workflows/ci.yml` — verification  
- [Telegram Bot API](https://core.telegram.org/bots/api) — upstream  
- [aiogram docs](https://docs.aiogram.dev/) — UX oracle  
