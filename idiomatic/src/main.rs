// 1. This code looks terrible. Let's start cleaning this up by running `cargo fmt`. If you
// configured your editor or IDE to run `cargo fmt` automatically upon save, you can just save!

// 2. `cargo fmt` is great, but it doesn't add blank lines where there are none. Go ahead and add
// some blank lines in places you think it would make sense.

// 3. Time to clean up! Run `cargo clippy`. Fix up all the warnings so `cargo clippy` is silent.

// Challenge: Clippy doesn't find *everything*. What else would you change to make this code better?

use std::f32::consts::PI;

fn count(num_to_count: i32) -> i32 {
    let mut result = 0;
    loop {
        if result > PI as i32 && result > num_to_count {
            break;
        }
        result += 1;
    }

    num_to_count
}

fn main() {
    println!("I can count to {}", count(5));
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_counting() {
        assert_eq!(count(5) == 5, true);
    }
}
