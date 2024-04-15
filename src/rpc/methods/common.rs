// Copyright 2019-2024 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

use crate::rpc::error::ServerError;
use crate::rpc::{ApiVersion, Ctx, RpcMethod};
use fvm_ipld_blockstore::Blockstore;
use once_cell::sync::Lazy;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::any::Any;
use uuid::Uuid;

static SESSION_UUID: Lazy<Uuid> = Lazy::new(Uuid::new_v4);

macro_rules! for_each_method {
    ($callback:ident) => {
        $callback!(crate::rpc::common::Session);
        $callback!(crate::rpc::common::Version);
        $callback!(crate::rpc::common::Shutdown);
        $callback!(crate::rpc::common::StartTime);
    };
}
pub(crate) use for_each_method;

/// The returned session UUID uniquely identifies the API node.
pub enum Session {}
impl RpcMethod<0> for Session {
    const NAME: &'static str = "Filecoin.Session";
    const PARAM_NAMES: [&'static str; 0] = [];
    const API_VERSION: ApiVersion = ApiVersion::V0;

    type Params = ();
    type Ok = Uuid;

    async fn handle(_: Ctx<impl Any>, (): Self::Params) -> Result<Uuid, ServerError> {
        Ok(*SESSION_UUID)
    }
}

pub enum Version {}
impl RpcMethod<0> for Version {
    const NAME: &'static str = "Filecoin.Version";
    const PARAM_NAMES: [&'static str; 0] = [];
    const API_VERSION: ApiVersion = ApiVersion::V0;

    type Params = ();
    type Ok = PublicVersion;

    async fn handle(ctx: Ctx<impl Blockstore>, (): Self::Params) -> Result<Self::Ok, ServerError> {
        let v = &*crate::utils::version::FOREST_VERSION;
        Ok(PublicVersion {
            version: crate::utils::version::FOREST_VERSION_STRING.clone(),
            api_version: ShiftingVersion::new(v.major, v.minor, v.patch),
            block_delay: ctx.state_manager.chain_config().block_delay_secs,
        })
    }
}

pub enum Shutdown {}
impl RpcMethod<0> for Shutdown {
    const NAME: &'static str = "Filecoin.Shutdown";
    const PARAM_NAMES: [&'static str; 0] = [];
    const API_VERSION: ApiVersion = ApiVersion::V0;

    type Params = ();
    type Ok = ();

    async fn handle(ctx: Ctx<impl Any>, (): Self::Params) -> Result<Self::Ok, ServerError> {
        ctx.shutdown.send(()).await?;
        Ok(())
    }
}

pub enum StartTime {}
impl RpcMethod<0> for StartTime {
    const NAME: &'static str = "Filecoin.StartTime";
    const PARAM_NAMES: [&'static str; 0] = [];
    const API_VERSION: ApiVersion = ApiVersion::V0;

    type Params = ();
    type Ok = chrono::DateTime<chrono::Utc>;

    async fn handle(ctx: Ctx<impl Blockstore>, (): Self::Params) -> Result<Self::Ok, ServerError> {
        Ok(ctx.start_time)
    }
}

/// Represents the current version of the API.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "PascalCase")]
pub struct PublicVersion {
    pub version: String,
    #[serde(rename = "APIVersion")]
    pub api_version: ShiftingVersion,
    pub block_delay: u32,
}

/// Integer based value on version information. Highest order bits for Major,
/// Mid order for Minor and lowest for Patch.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ShiftingVersion(u32);

impl ShiftingVersion {
    pub const fn new(major: u64, minor: u64, patch: u64) -> Self {
        Self((major as u32) << 16 | (minor as u32) << 8 | (patch as u32))
    }
}