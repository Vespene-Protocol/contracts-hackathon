#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use std::time::{SystemTime};
use crate::error::ContractError;
use crate::msg::{CountResponse, ExecuteMsg, InstantiateMsg, QueryMsg, CollateralBalance, BorrowBalance};
use crate::state::{State, STATE};

const PERCENTAGE: f64 = (19.49/100.00);
const PERCENTAGE_BORROW: f64 = ((19.49+9.00)/100.00);

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let now = SystemTime::now();
    let state = State {
        uust: info.funds[0].amount,
        owner: info.sender.clone(),
        interest: PERCENTAGE,
        timestamp: now,
        borrowed: None,
        interest_borrow: None
    };
    STATE.save(deps.storage, &state)?;

    Ok(Response::new().add_attribute("method", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Borrow {} => try_borrow(deps, info, msg)
    }
}

pub fn try_borrow(deps: DepsMut, info: MessageInfo, msg: ExecuteMsg) -> Result<Response, ContractError> {
    let now = SystemTime::now();
    let uust = info.funds[0].amount;
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        if info.sender != state.owner {
            return Err(ContractError::Unauthorized {});
        }
        if uust > state.uust {
            return Err(ContractError::NoFunds {});
        }
        state.uust -= uust;
        state.borrowed = Some(uust);
        state.interest_borrow = Some(PERCENTAGE_BORROW);
        state.timestamp = now;
        Ok(state)
    })?;

    Ok(Response::new().add_attribute("method", "try_borrow"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::CollateralBalance {} => to_binary(&query_collateral_balance(deps)?),
        QueryMsg::BorrowBalance {} => to_binary(&query_borrow_balance(deps)?),
    }
}

fn query_collateral_balance(deps: Deps) -> StdResult<CollateralBalance> {
    let state = STATE.load(deps.storage)?;
    Ok(CollateralBalance { balance: state.uust, timestamp: state.timestamp, interest: state.interest })
}

fn query_borrow_balance(deps: Deps) -> StdResult<BorrowBalance> {
    let state = STATE.load(deps.storage)?;
    Ok(BorrowBalance { balance: state.uust, timestamp: state.timestamp, interest: state.interest_borrow })
}