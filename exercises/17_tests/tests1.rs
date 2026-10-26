// Tests are important to ensure that your code does what you think it should
// do.

fn is_even(n: i64) -> bool {
    n % 2 == 0
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::is_even;

    #[test]
    fn you_can_assert() {
        assert!(is_even(1) == false);
        assert!(is_even(2) == true);
    }
}
