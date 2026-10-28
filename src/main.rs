// SPDX-FileCopyrightText: Copyright Logically Magic Inc. <hello@logicallymagic.com>
// SPDX-FileCopyrightText: Copyright Hamid Jahanzad <Hamid@Jahanzad.com>
// SPDX-FileCopyrightText: Copyright Contributors to the Bank8:OpenCA
// SPDX-License-Identifier: Apache-2.0

//! Sentin8 Bank8:OpenCA - Canadian open banking, the Canadian
//! consumer-driven-banking profile of the FDX v6.x provider/recipient
//! wire. See `DESIGN.md` (OBARCH-4).
//!
//! This service is a thin country adapter: it embeds
//! `bank8-open-core` (the jurisdiction-agnostic kernel - OBARCH-1)
//! and supplies the Canadian profile descriptor over the shared FDX
//! wire (`core::wire::fdx`) and the shared FDX ceremony (DCR + PAR +
//! PKCE + DPoP), with its own version cadence
//! (`BANK8_OPEN_CA_VERSION`) and its own schema, registration, and
//! per-environment enablement.
//!
//! Skeleton - the OBARCH-1 core extraction and the shared FDX wire
//! land first; this binary then takes the extracted handlers, the
//! version-parameterized route contract, and its registration
//! data. The FDX spec is NEVER published here (OBARCH-2): the
//! published `docs/openapi.json` carries bank8-owned routes only;
//! customers supply the licensed FDX API specification under
//! gitignored `specs/fdx/{version}/` (see `specs/README.md`).

use utoipa::OpenApi;

mod openapi;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    sentin8::shell::serve(
        sentin8::shell::Sentin8Shell::new("bank8:open-ca", "BANK8_OPEN_CA")
            .with_registration(print_registration_stub)
            .with_own_health()
            .with_openapi(openapi::CaOpenApi::openapi()),
    )
    .await
}

/// Registration stub - the full registration data (policy sets,
/// Shield gates `bank8:open-ca:*`, admin UI pages, SQL migrations)
/// lands with the OBARCH-1 extraction; the shape follows the
/// `bank8-open-us` registration (per-jurisdiction identity).
fn print_registration_stub(section: Option<String>) -> anyhow::Result<()> {
    // Placeholder until the registration module lands (OBARCH-3).
    let _ = section;
    Ok(())
}
