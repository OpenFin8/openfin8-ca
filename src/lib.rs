// SPDX-FileCopyrightText: Copyright Logically Magic Inc. <hello@logicallymagic.com>
// SPDX-FileCopyrightText: Copyright Hamid Jahanzad <Hamid@Jahanzad.com>
// SPDX-FileCopyrightText: Copyright Contributors to the Bank8:OpenCA
// SPDX-License-Identifier: Apache-2.0

//! Library target for the bank8-open-ca adapter: the registration and
//! spec modules for the Canadian profile. The implementation itself
//! lives in `bank8-open-core` (shared with bank8-open-us - OBARCH-4);
//! integration tests run from the core crate.

pub mod openapi;
pub mod registration;
