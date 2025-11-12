use crate::state::PrizeSplit;

pub fn calculate_prizes(split: &PrizeSplit, total_pool: u64) -> Vec<u64> {
    match split {
        PrizeSplit::WinnerTakesAll => vec![total_pool],
        PrizeSplit::TopThree { first, second, third: _ } => {
            let first_amt = total_pool * (*first as u64) / 100;
            let second_amt = total_pool * (*second as u64) / 100;
            let third_amt = total_pool.checked_sub(first_amt + second_amt).unwrap_or(0);
            vec![first_amt, second_amt, third_amt]
        },
        PrizeSplit::TopFive { first, second, third, fourth, fifth: _ } => {
            let one = total_pool * (*first as u64) / 100;
            let two = total_pool * (*second as u64) / 100;
            let three = total_pool * (*third as u64) / 100;
            let four = total_pool * (*fourth as u64) / 100;
            let five = total_pool.checked_sub(one + two + three + four).unwrap_or(0);
            vec![one, two, three, four, five]
        },
    }
}
