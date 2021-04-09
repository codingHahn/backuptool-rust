fn main() {
    println!("Hello, world!");
}

// Tests:
#[cfg(test)]
mod tests {
    #[test]
    fn test_1() {
        assert_eq!(1, 3);
    }
}
