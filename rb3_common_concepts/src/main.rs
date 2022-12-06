fn main() {
    // Convert temperatures between Fahrenheit and Celsius.
    fn convert_to_fahrenheit(celsius: f32) -> f32 {
        (9.0 / 5.0) * celsius + 32.0
    }

    assert_eq!(convert_to_fahrenheit(20.0), 68.0);
    assert_eq!(convert_to_fahrenheit(0.0), 32.0);

    // Generate the nth Fibonacci number.
    fn generate_nth_fibonnaci(nth: u32) -> u32 {
        let num = match nth {
            0 => 0,
            1 => 1,
            _ => generate_nth_fibonnaci(nth - 1) + generate_nth_fibonnaci(nth - 2),
        };

        return num;
    }

    assert_eq!(generate_nth_fibonnaci(0), 0);
    assert_eq!(generate_nth_fibonnaci(1), 1);
    assert_eq!(generate_nth_fibonnaci(2), 1);
    assert_eq!(generate_nth_fibonnaci(4), 3);
    assert_eq!(generate_nth_fibonnaci(5), 5);
    assert_eq!(generate_nth_fibonnaci(8), 21);
}
