// SPDX-FileCopyrightText: Copyright Logically Magic Inc. <hello@logicallymagic.com>
// SPDX-FileCopyrightText: Copyright Hamid Jahanzad <Hamid@Jahanzad.com>
// SPDX-FileCopyrightText: Copyright Contributors to the Bank8:OpenCA
// SPDX-License-Identifier: Apache-2.0

//! Sentin8 Bank8:OpenCA - Canadian open banking: the Canadian
//! consumer-driven-banking profile of the FDX v6.x provider/recipient
//! wire. See `DESIGN.md` (OBARCH-4).
//!
//! This is a THIN COUNTRY ADAPTER (the design's no-fork guarantee):
//! the entire implementation - consent ledger, recipient registry,
//! CSDF minimization, notification worker, provider wire, consumers -
//! lives in `bank8-open-core`, shared byte-for-byte with
//! `bank8-open-us`. This binary contributes exactly three things:
//! the Canadian [`Profile`](bank8_open_core::profile::Profile)
//! (jurisdiction identity, scope vocabulary, schema `open_ca`,
//! version from `BANK8_OPEN_CA_VERSION`), its registration data
//! (`registration.rs`), and its published-spec policy (`openapi.rs` -
//! bank8-owned routes only; the FDX API specification is licensed and
//! never published here, OBARCH-2).

use utoipa::OpenApi;

mod openapi;
mod registration;

use bank8_open_core::profile::Profile;
use bank8_open_core::server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let profile = Profile::canada();
    sentin8::shell::serve(
        sentin8::shell::Sentin8Shell::new(profile.service_type, profile.env_prefix)
            .with_registration(registration::print)
            .with_state(|core| server::build_state(core, Profile::canada()))
            .with_own_health()
            .with_router(server::routes)
            .with_openapi(openapi::CaOpenApi::openapi()),
    )
    .await
}
