// cargo fmt && cargo test

// To be completely transparent this is not a code golf task but it fits the formula well.
use chrono::prelude::*;

struct Call {
    start: DateTime<Utc>,
    end: DateTime<Utc>,
}

pub fn main() {
    let t1_start = Utc.ymd(2000, 6, 1).and_hms(0, 0, 0);
    let t1_end = Utc.ymd(2000, 6, 1).and_hms(0, 0, 0);

    let c1 = Call {
        start: t1_start,
        end: t1_end,
    };
    println!("Hello overlapping_calls");
}

fn calculate_max_concurrent_calls(calls: Vec<Call>) -> i32 {
    let mut max_calls = 0;
    max_calls
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_calls_means_zero_concurrent_calls() {
        let actual = calculate_max_concurrent_calls(vec![]);
        assert_eq!(actual, 0);
    }
}
