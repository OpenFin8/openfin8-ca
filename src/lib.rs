// SPDX-FileCopyrightText: Copyright Logically Magic Inc. <hello@logicallymagic.com>
// SPDX-FileCopyrightText: Copyright Hamid Jahanzad <Hamid@Jahanzad.com>
// SPDX-FileCopyrightText: Copyright Contributors to the OpenFin8:CA
// SPDX-License-Identifier: Apache-2.0

//! Library target for the openfin8-ca adapter: the registration and
//! spec modules for the Canadian profile. The implementation itself
//! lives in `openfin8-core` (shared with openfin8-us -);
//! integration tests run from the core crate.

pub mod openapi;

// The utoipa dump for registration::print - mirrored from main.rs; the
// lib crate compiles registration::print too (test harnesses embedding
// via #[path] never invoke it, so the pointer is referenced, not run).
#[cfg_attr(test, allow(dead_code))]
static OPENAPI_DUMP: fn() -> serde_json::Value = || {
    use utoipa::OpenApi as _;
    serde_json::to_value(openapi::CaOpenApi::openapi()).unwrap_or(serde_json::Value::Null)
};

pub mod registration;
