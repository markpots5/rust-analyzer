fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_hello_world() {
        main();
    }

    #[test]
    fn test_basic_math() {
        assert_eq!(2 + 2, 4);
    }

    #[cfg(test)]
    mod tests2 {
        use super::*;

        #[test]
        fn test_print_hello_world() {
            main();
        }

        #[test]
        fn test_basic_math() {
            assert_eq!(2 + 2, 4);
        }
    }
}
