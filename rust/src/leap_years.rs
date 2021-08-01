// compile with: rustc leap_years.rs
// run with ./leap_years

// In the Gregorian calendar, a leap year is created by extending February to 29 days in order to keep the calendar year synchronized with the astronomical year. These longer years occur in years which are multiples of 4, with the exception of centennial years that aren’t multiples of 400.
// Write a program to print all the leap years from the year 1800 up to and including 2400.

pub fn main() {
    for i in 1800..2401 {
        let a = is_leap(i);
        if a {
            println!("{}", i);
        }
    };
}

fn is_leap(y: i32) -> bool {
    if y % 100 == 0 {
        if y % 400 == 0 {
            return true;
        }
        return false;
    } else if y % 4 == 0 {
        return true;
    }
    false
}

