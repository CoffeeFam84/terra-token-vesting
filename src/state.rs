use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::CanonicalAddr;
use cw_storage_plus::{Item, Map};

use crate::msg::Schedule;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub vesting_token: CanonicalAddr,
    pub vesting_manager: CanonicalAddr,
}

pub const CONFIGURATION: Item<Config> = Item::new("config");
pub const SCHEDULES: Map<&[u8], Schedule> = Map::new("vesting_schedules");