#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

use cosmwasm_std:: {
  attr, from_binary, Addr, DepsMut, Env, MessageInfo, Response, StdResult, StdError, Uint128
};

use cw20::Cw20ReceiveMsg;
use terraswap::asset::{AssetInfo, Asset};
use crate::{
  msg::{ InstantiateMsg, ExecuteMsg, Cw20HookMsg },
  state:: { Config, CONFIGURATION, SCHEDULES },
  utils:: {get_pending_payout}
};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate (
  deps: DepsMut,
  env: Env,
  _info: MessageInfo,
  msg: InstantiateMsg,
) -> StdResult<Response> {
  CONFIGURATION.save(
    deps.storage,
    &Config {
      vesting_token: deps.api.addr_canonicalize(&msg.tokenAddress)?,
      vesting_manager: deps.api.addr_canonicalize(&_info.sender.to_string())?,
    }
  )?;

  Ok(Response::default())
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
  deps: DepsMut,
  env: Env,
  _info: MessageInfo,
  msg: ExecuteMsg
) -> StdResult<Response> {
  match msg {
    ExecuteMsg::Receive(msg) => receive_cw20(deps, env, _info, msg),
    ExecuteMsg::ReleaseToken {} => release(deps, env, _info.sender.to_string()),
  }
}

pub fn receive_cw20(
  deps: DepsMut,
  env: Env,
  info: MessageInfo,
  cw20_msg: Cw20ReceiveMsg,
) -> StdResult<Response> {
  match from_binary(&cw20_msg.msg)? {
    Cw20HookMsg::AddSchedule {
      beneficiary,
      amount,
      lock_period,
      release_period,
    } => {
      let config = CONFIGURATION.load(deps.storage)?;
      if let token_addr = config.vesting_token {
        if deps.api.addr_humanize(&token_addr)? == info.sender.clone() {
          return add_schedule(deps, env, cw20_msg.amount, beneficiary, lock_period.u64(), release_period.u64())
        }
      }
      Err(StdError::generic_err("Invalid Cw20 Token"))
    }
  }
}

pub fn add_schedule(
  deps: DepsMut,
  env: Env,
  amount: Uint128,
  beneficiary: String,
  lock_period: u64,
  release_period: u64
) -> StdResult<Response> {
  if amount.is_zero() {
    return Err(StdError::generic_err("amount is zero"));
  }

  let current_time = env.block.time.seconds();

  let mut schedule = SCHEDULES
    .load( 
      deps.storage, 
      deps.api.addr_canonicalize(&beneficiary)?.as_slice(),
    )
    .unwrap_or_default();
    
    schedule.payout = amount;
    schedule.vesting = release_period * 86400 * 30;
    schedule.vesting_start = current_time + lock_period * 86400 * 30;
    schedule.last_time = schedule.vesting_start;
    SCHEDULES.save(
      deps.storage,
      deps.api.addr_canonicalize(&beneficiary)?.as_slice(),
      &schedule,
    );

    Ok(Response::default())
}

pub fn release(
  deps: DepsMut,
  env: Env,
  user: String
) -> StdResult<Response> {
  let mut schedule = SCHEDULES.load(deps.storage, deps.api.addr_canonicalize(&user)?.as_slice())?;

  let current_time = env.block.time.seconds();

  if current_time < schedule.vesting_start {
    return Err(StdError::generic_err("lock period"));
  }
  
  let time_since_last = current_time - schedule.last_time;
  let payout = get_pending_payout(schedule.clone(), time_since_last);

  if payout.is_zero() {
    return Err(StdError::generic_err("nothing to release"))
  }

  schedule.payout = schedule.payout.checked_sub(payout)?;
  if schedule.payout.is_zero() {
    SCHEDULES.remove(deps.storage, deps.api.addr_canonicalize(&user)?.as_slice());
  } else {
    schedule.vesting = schedule.vesting - time_since_last;
    schedule.last_time = current_time;
    SCHEDULES.save(
      deps.storage,
      deps.api.addr_canonicalize(&user)?.as_slice(),
      &schedule,
    )?;
  }

  let config = CONFIGURATION.load(deps.storage)?;
  let asset = Asset {
    info: AssetInfo::Token {
      contract_addr: deps.api.addr_humanize(&config.vesting_token)?.to_string(),
    },
    amount: payout,
  };

  Ok(Response::new()
    .add_message(asset.into_msg(&deps.querier, Addr::unchecked(user))?)
    .add_attributes(vec![
      attr("action", "release"),
      attr("amount", payout.to_string()),
    ]))
}