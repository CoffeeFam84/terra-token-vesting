
use cosmwasm_std::{ Decimal, Uint128}
use crate::msg::Schedule;

pub fn get_pending_payout(schedule: Schedule, time_since_last: u64) -> Uint128 {
  let mut payout = schedule.payout * 
                              Decimal * from_ratio(
                                Uint128::from(time_since_last as u128),
                                Uint128::from(schedule.vesting as u128),
                              );
  if payout > schedule.payout {
    payout = schedule.payout;
  }

  payout
}