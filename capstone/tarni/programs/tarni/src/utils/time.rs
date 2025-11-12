use anchor_lang::prelude::*;

pub fn now_ts() -> i64 {
    Clock::get().unwrap().unix_timestamp
}
