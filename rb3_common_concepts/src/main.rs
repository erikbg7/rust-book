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

    // Generate The Twelve Days of Christmas song.
    const ITEMS: [&str; 12] = [
        "a partridge in a pear tree",
        "two turtle doves",
        "three French hens",
        "four calling birds",
        "five gold rings",
        "six geese a-laying",
        "seven swans a-swimming",
        "eight maids a-milking",
        "nine ladies dancing",
        "ten lords a-leaping",
        "eleven pipers piping",
        "twelve drummers drumming",
    ];

    for day in 1..=12 {
        println!("\nOn the {} day of Christmas my true love sent to me", day);
        for i in 1..=day {
            println!("{}", ITEMS[day - i]);
        }
    }
}
