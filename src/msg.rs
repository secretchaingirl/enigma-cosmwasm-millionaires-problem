use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm::types::{HumanAddress};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum HandleMsg {
	  AddMillionnaire { address: HumanAddress, net_worth: u8 },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum QueryMsg {
    ComputeRichest {},
}

// We define a custom struct for the ComputeRichest query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ComputeRichestResponse {
    pub address: HumanAddress,
}
