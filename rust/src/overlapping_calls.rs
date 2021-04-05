// cargo fmt && cargo test

// To be completely transparent this is not a code golf task but it fits the formula well.
use chrono::prelude::*;

struct Call {
    start: DateTime<Utc>,
    end: DateTime<Utc>,
}

pub fn main() {
    println!("Hello from Overlapping Calls! Use tests for algorithm demonstration.");
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

    #[test]
    fn max_from_multiple_calls_is_calculated_correctly() {
        let c1 = Call {
            start: Utc.ymd(2000, 1, 1).and_hms(6, 0, 0),
            end: Utc.ymd(2000, 1, 1).and_hms(7, 0, 0),
        };
        let c2 = Call {
            start: Utc.ymd(2000, 1, 1).and_hms(5, 0, 0),
            end: Utc.ymd(2000, 1, 1).and_hms(8, 0, 0),
        };
        let calls = vec![c1, c2];

        let actual = calculate_max_concurrent_calls(calls);

        assert_eq!(actual, 2);
    }
}
