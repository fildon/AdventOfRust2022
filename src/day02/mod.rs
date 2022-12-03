pub fn add_three(a: i32) -> i32 {
    internal_adder(a, 3)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, add_three(1));
    }
}
