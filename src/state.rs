use cosmwasm_std::Addr;
use cw_storage_plus::Map;

// username -> address
// username -> NFT already handled as token_id == username
pub const NAMES: Map<&str, Addr> = Map::new("names");
