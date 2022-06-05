use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cw20::Cw20ReceiveMsg;
use cosmwasm_std::{Uint128, Uint64};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {
  pub tokenAddress: String
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
  Receive(Cw20ReceiveMsg),
  ReleaseToken {}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema, Default)]
#[serde(rename_all = "snake_case")]
pub struct Schedule {
  pub payout: Uint128,
  pub vesting: u64,
  pub vesting_start: u64,
  pub last_time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Cw20HookMsg {
  AddSchedule {
    beneficiary: String,
    amount: Uint128,
    lock_period: Uint64,
    release_period: Uint64,
  },
}