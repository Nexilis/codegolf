// compile with: rustc abundant_numbers.rs
// run with ./abundant_numbers


// An abundant number is a number for which the sum of its proper divisors (divisors not including the number itself) is greater than the number itself. For example 12 is abundant because its proper divisors are 1, 2, 3, 4, and 6 which add up to 16.
// Print all the abundant numbers from 1 to 200 inclusive, each on their own line.

pub fn main() {
    for i in 0..201 {
        let a = is_abundant(i);
        if a {
            println!("{}", i);
        }
    };
}

fn is_abundant(num: i32) -> bool {
    let mut sum = 0;

    for i in 1..(num-1) {
        if num % i == 0 {
            sum += i;
        }
    }

    if sum > num {
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn confirm_a_few_abundant_numbers() {
        let a1 = is_abundant(11);
        let a2 = is_abundant(12);
        let a3 = is_abundant(84);
        let a4 = is_abundant(174);
        let a5 = is_abundant(199);

        assert_eq!(a1, false, "assert 11");
        assert_eq!(a2, true, "assert 12");
        assert_eq!(a3, true, "assert 84");
        assert_eq!(a4, true, "assert 174");
        assert_eq!(a5, false, "assert 199");
    }
}
