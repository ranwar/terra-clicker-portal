use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std:: Addr;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub speed: i32, // Changed from count to speed
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
  // We won't need Increment and Reset from the template so we can remove them
  // Increment {},
  // Reset { speed: i32 }, // Changed from count to speed

  UpsertScore { score: u16 }, // This will add or update scores
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
  // GetSpeed returns the speed of light as a json-encoded number
  GetSpeed {}, // Changed from GetCount to GetSpeed
  // Just like with GetSpeed, we declare an enum for scores too
  GetScores {},
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SpeedResponse { // Changed from CountResponse to SpeedResponse
  pub speed: i32, // Changed from count to speed
}

// Here's the response type for our scores, it's a vector of the same object type we defined in state.rs
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ScoreResponse{
  pub scores: Vec<(Addr, u16)>,
}