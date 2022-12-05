use std::{cmp, fs::File, io::Read, path::Path};

fn main() {
    let path = Path::new("src/test");
    let mut file = String::new();

    File::open(path)
        .expect("The file should exist")
        .read_to_string(&mut file)
        .expect("The file should be readable");

    let mut most_calories = 0;
    let mut current_calories = 0;

    file.lines().for_each(|l| {
        match l.trim().parse::<i32>() {
            Ok(n) => current_calories += n,
            Err(_) => {
                most_calories = cmp::max(most_calories, current_calories);
                current_calories = 0;
            }
        };
    });

    println!("The most calories is {}", most_calories)
}

