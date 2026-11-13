// SPDX-FileCopyrightText: Copyright Logically Magic Inc. <hello@logicallymagic.com>
// SPDX-FileCopyrightText: Copyright Hamid Jahanzad <Hamid@Jahanzad.com>
// SPDX-FileCopyrightText: Copyright Contributors to the OpenFin8:CA
// SPDX-License-Identifier: Apache-2.0

//! Sentin8 OpenFin8:CA - Canadian open banking: the Canadian
//! consumer-driven-banking profile of the FDX v6.x provider/recipient
//! wire. See `DESIGN.md`).
//!
//! This is a THIN COUNTRY ADAPTER (the design's no-fork guarantee):
//! the entire implementation - consent ledger, recipient registry,
//! CSDF minimization, notification worker, provider wire, consumers -
//! lives in `openfin8-core`, shared byte-for-byte with
//! `openfin8-us`. This binary contributes exactly three things:
//! the Canadian [`Profile`](openfin8_core::profile::Profile)
//! (jurisdiction identity, scope vocabulary, schema `openfin8_ca`,
//! version from `OPENFIN8_CA_VERSION`), its registration data
//! (`registration.rs`), and its published-spec policy (`openapi.rs` -
//! OpenFin8-owned routes only; the FDX API specification is licensed and
//! never published here,).

use utoipa::OpenApi;

mod openapi;

// The utoipa dump as a fn pointer for registration.rs (which cannot
// name crate::openapi - embedded test harnesses have no openapi module;
// see registration.rs and the mirrored static in lib.rs).
#[cfg_attr(test, allow(dead_code))]
static OPENAPI_DUMP: fn() -> serde_json::Value = || {
    use utoipa::OpenApi as _;
    serde_json::to_value(openapi::CaOpenApi::openapi()).unwrap_or(serde_json::Value::Null)
};

mod registration;

use openfin8_core::profile::Profile;
use openfin8_core::server;

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
