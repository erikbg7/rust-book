pub mod triangle;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let r1 = Rectangle {
        width: 20,
        height: 10,
    };

    fn area_with_reference(r: &Rectangle) -> u32 {
        return r.width * r.height;
    }

    fn area_with_ownership(r: Rectangle) -> u32 {
        return r.width * r.height;
    }

    println!("Result {}", area_with_reference(&r1));
    println!("Rectangle is not consumed {}", r1.width);

    println!("Result {}", area_with_ownership(r1));

    // Code above tries to use a value that has been already consumed,
    // so we have an error: value borrowed here after move
    // println!("Rectangle is not consumed {}", r1.width);

    //////////////////////////////

    let r2 = Rectangle {
        width: 20,
        height: 10,
    };

    let a2 = area_with_reference(&r2);
    assert_eq!(a2, 200);

    let r3 = Rectangle { width: 10, ..r2 };

    let a3 = area_with_reference(&r3);
    assert_eq!(a3, 100);

    // We can still use r2 because values implement Copy trait
    println!("r2 w {}", r2.width);

    println!("r2 struct, {:?}", r2);

    triangle::method_example();
}
