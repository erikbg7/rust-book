fn main() {
    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            println!("what is {:?}", self);
        }
    }

    let m = Message::Write(String::from("Any string"));
    m.call();

    //////////////
    // Option Test
    //////////////

    let num1 = 30;
    let num2: Option<i32> = Some(30);

    // let sum = num1 + num2; // cannot sum i32 and Option<i32>

    let sum_unwrapped = num1 + num2.unwrap();
    // println!("num2 {}", num2); // num2 is consumed by unwrap

    let sum_match = match num2 {
        Some(n) => num1 + n,
        None => num1,
    };

    assert_eq!(sum_unwrapped, 60);
    assert_eq!(sum_match, 60);
}
