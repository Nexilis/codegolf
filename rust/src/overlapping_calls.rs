// cargo fmt && cargo test

// To be completely transparent this is not a code golf task but it seems to be an interesting challenge.
#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use chrono::prelude::{DateTime, TimeZone, Utc};
use std::cmp::Ordering;

struct Call {
    start: DateTime<Utc>,
    end: DateTime<Utc>,
}

#[derive(Eq)]
enum CallEvent {
    Start(DateTime<Utc>),
    End(DateTime<Utc>),
}

impl Ord for CallEvent {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (CallEvent::Start(x), CallEvent::Start(y))
            | (CallEvent::End(x), CallEvent::End(y))
            | (CallEvent::Start(x), CallEvent::End(y)) => x.cmp(y),
            (CallEvent::End(x), CallEvent::Start(y)) => {
                if x.cmp(y) == Ordering::Equal {
                    return Ordering::Less;
                }
                x.cmp(y)
            }
        }
    }
}

impl PartialOrd for CallEvent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for CallEvent {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (CallEvent::Start(x), CallEvent::Start(y)) => x == y,
            (CallEvent::End(x), CallEvent::End(y)) => x == y,
            _ => false,
        }
    }
}

pub fn main() {
    println!("Hello from Overlapping Calls! Use tests for algorithm demonstration.");
}

fn calculate_max_concurrent_calls(calls: Vec<Call>) -> i32 {
    let mut all_call_dates = calls.iter().fold(Vec::new(), |mut agg, val| {
        agg.push(CallEvent::Start(val.start));
        agg.push(CallEvent::End(val.end));
        agg
    });
    all_call_dates.sort();

    let mut max_calls = 0;
    let mut current_calls = 0;
    for call in all_call_dates {
        match call {
            CallEvent::Start(_) => {
                current_calls += 1;
                if current_calls > max_calls {
                    max_calls = current_calls;
                }
            }
            CallEvent::End(_) => current_calls -= 1,
        }
    }

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
    fn call_end_in_the_same_time_as_another_call_start_is_a_concurrent_call() {
        let c1 = Call {
            start: Utc.ymd(2000, 1, 1).and_hms(2, 0, 0),
            end: Utc.ymd(2000, 1, 1).and_hms(3, 0, 0),
        };
        let c2 = Call {
            start: Utc.ymd(2000, 1, 1).and_hms(1, 0, 0),
            end: Utc.ymd(2000, 1, 1).and_hms(2, 0, 0),
        };
        let calls = vec![c1, c2];

        let actual = calculate_max_concurrent_calls(calls);

        assert_eq!(actual, 1);
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
        let c3 = Call {
            start: Utc.ymd(2000, 1, 1).and_hms(7, 30, 0),
            end: Utc.ymd(2000, 1, 1).and_hms(9, 0, 0),
        };
        let c4 = Call {
            start: Utc.ymd(2010, 1, 1).and_hms(5, 0, 0),
            end: Utc.ymd(2010, 1, 1).and_hms(8, 0, 0),
        };
        let calls = vec![c1, c2, c3, c4];

        let actual = calculate_max_concurrent_calls(calls);

        assert_eq!(actual, 2);
    }
}
