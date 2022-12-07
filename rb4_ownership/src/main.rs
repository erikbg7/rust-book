fn main() {
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

    let mut song = String::from("");

    for day in 1..=12 {
        song += &format!("\n\nOn the {day} day of Christmas my true love sent to me");
        for i in 1..=day {
            song += &format!("\n {}", ITEMS[day - i]);
        }
    }

    println!("{song}");
}
