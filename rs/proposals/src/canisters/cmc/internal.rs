//! CMC types exposed outside the canister but NOT mentioned in the CMC .did file.
//! From: rs/nns/cmc/src/lib.rs
use candid::{self, CandidType, Decode, Deserialize, Encode, Principal};
use serde::Serialize;

use ic_base_types::SubnetId;

/// From: rs/nns/cmc/src/lib.rs
/// Argument taken by the set_authorized_subnetwork_list endpoint
#[derive(Serialize, Deserialize, CandidType, Clone, Hash, Debug, PartialEq, Eq)]
pub struct SetAuthorizedSubnetworkListArgs {
    pub who: Option<Principal>,
    pub subnets: Vec<SubnetId>,
}

#[derive(Serialize, Deserialize, CandidType, Clone, Hash, Debug, PartialEq, Eq)]
pub enum UpdateSubnetTypeArgs {
    Add(String),
    Remove(String),
}

#[derive(Serialize, Deserialize, CandidType, Clone, Hash, Debug, PartialEq, Eq)]
pub enum ChangeSubnetTypeAssignmentArgs {
    Add(SubnetListWithType),
    Remove(SubnetListWithType),
}

#[derive(Serialize, Deserialize, CandidType, Clone, Hash, Debug, PartialEq, Eq)]
pub struct SubnetListWithType {
    pub subnets: Vec<SubnetId>,
    pub subnet_type: String,
}
