use crate::state::PrizeSplit;

pub fn calculate_prizes(split: &PrizeSplit, total_pool: u64) -> Vec<u64> {
    match split {
        PrizeSplit::WinnerTakesAll => {
            vec![total_pool]
        }
        PrizeSplit::TopThree { first, second , third: _} => {
            let first_amt = total_pool
                .checked_mul(*first as u64)
                .and_then(|v| v.checked_div(100))
                .unwrap_or(0);
            
            let second_amt = total_pool
                .checked_mul(*second as u64)
                .and_then(|v| v.checked_div(100))
                .unwrap_or(0);
            
            let third_amt = total_pool
                .checked_sub(first_amt)
                .and_then(|v| v.checked_sub(second_amt))
                .unwrap_or(0);
            
            vec![first_amt, second_amt, third_amt]
        }
        PrizeSplit::TopFive { first, second, third, fourth, fifth: _} => {
            let first_amt = total_pool.checked_mul(*first as u64).and_then(|v| v.checked_div(100)).unwrap_or(0);
            let second_amt = total_pool.checked_mul(*second as u64).and_then(|v| v.checked_div(100)).unwrap_or(0);
            let third_amt = total_pool.checked_mul(*third as u64).and_then(|v| v.checked_div(100)).unwrap_or(0);
            let fourth_amt = total_pool.checked_mul(*fourth as u64).and_then(|v| v.checked_div(100)).unwrap_or(0);
            
            let fifth_amt = total_pool
                .checked_sub(first_amt)
                .and_then(|v| v.checked_sub(second_amt))
                .and_then(|v| v.checked_sub(third_amt))
                .and_then(|v| v.checked_sub(fourth_amt))
                .unwrap_or(0);
            
            vec![first_amt, second_amt, third_amt, fourth_amt, fifth_amt]
        }
    }
}
