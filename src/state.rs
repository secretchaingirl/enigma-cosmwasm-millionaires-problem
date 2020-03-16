use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm::traits::Storage;
use cosmwasm::types::HumanAddress;
use cw_storage::{singleton, singleton_read, ReadonlySingleton, Singleton};

pub static MILLIONAIRES: &[u8] = b"millionaires";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Millionaire {
    pub address: HumanAddress,
    pub net_worth: u8,
}

pub fn millionaires<S: Storage>(storage: &mut S) -> Singleton<S, State> {
    singleton(storage, MILLIONAIRES)
}

pub fn millionaires_read<S: Storage>(storage: &S) -> ReadonlySingleton<S, State> {
    singleton_read(storage, MILLIONAIRES)
}
