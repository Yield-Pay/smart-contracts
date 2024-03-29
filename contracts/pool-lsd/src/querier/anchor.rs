//use cosmwasm_bignumber::{Decimal256, Uint256};
use cosmwasm_std::*;
use cw20::Cw20ExecuteMsg;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Config {},
    EpochState {
        block_height: Option<u64>,
        distributed_interest: Option<Uint256>,
    },
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct ConfigResponse {
    pub owner_addr: String,
    pub aterra_contract: String,
    pub interest_model: String,
    pub distribution_model: String,
    pub overseer_contract: String,
    pub collector_contract: String,
    pub distributor_contract: String,
    pub stable_denom: String,
    pub max_borrow_factor: Decimal,
}

pub fn config(deps: Deps, market: &CanonicalAddr) -> StdResult<ConfigResponse> {
    let market_config: ConfigResponse =
        deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: deps.api.addr_humanize(market).unwrap().to_string(),
            msg: to_binary(&QueryMsg::Config {})?,
        }))?;

    Ok(market_config)
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct EpochStateResponse {
    pub exchange_rate: Decimal,
    pub aterra_supply: Uint128,
}

pub fn epoch_state(deps: Deps, market: &CanonicalAddr) -> StdResult<EpochStateResponse> {
    let epoch_state: EpochStateResponse =
        deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: deps.api.addr_humanize(market).unwrap().to_string(),
            msg: to_binary(&QueryMsg::EpochState {
                block_height: None,
                distributed_interest: None,
            })?,
        }))?;

    Ok(epoch_state)
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    DepositStable {},
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Cw20HookMsg {
    /// Return stable coins to a user
    /// according to exchange rate
    RedeemStable {},
}

pub fn deposit_stable_msg(
    deps: Deps,
    market: &CanonicalAddr,
    denom: &str,
    amount: Uint128,
) -> StdResult<Vec<CosmosMsg>> {
    Ok(vec![CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: deps.api.addr_humanize(market).unwrap().to_string(),
        msg: to_binary(&HandleMsg::DepositStable {})?,
        funds: vec![Coin {
            denom: denom.to_string(),
            amount,
        }],
    })])
}

pub fn redeem_stable_msg(
    deps: Deps,
    market: &CanonicalAddr,
    token: &CanonicalAddr,
    amount: Uint128,
) -> StdResult<Vec<CosmosMsg>> {
    Ok(vec![CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: deps.api.addr_humanize(token).unwrap().to_string(),
        msg: to_binary(&Cw20ExecuteMsg::Send {
            contract: deps.api.addr_humanize(market).unwrap().to_string(),
            amount,
            msg: to_binary(&Cw20HookMsg::RedeemStable {}).unwrap(),
        })?,
        funds: vec![],
    })])
}
