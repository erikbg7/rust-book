#[derive(Debug)]
struct Triangle {
    width: u32,
    height: u32,
}

impl Triangle {
    fn area(&self) -> u32 {
        return self.width * self.height / 2;
    }

    fn can_hold(&self, t: &Triangle) -> bool {
        return self.width > t.width && self.height > t.height;
    }

    fn with_area(width: u32, area: u32) -> Self {
        return Self {
            width,
            height: area * 2 / width,
        };
    }
}

pub(crate) fn method_example() {
    let t1 = Triangle {
        width: 20,
        height: 10,
    };

    let t2 = Triangle {
        width: 40,
        height: 20,
    };

    let t3 = Triangle {
        width: 10,
        height: 5,
    };

    assert_eq!(t1.can_hold(&t2), false, "t2 does not fit in t1");
    assert_eq!(t1.can_hold(&t3), true, "t3 fits in t1");

    println!("Triangle res {}", t1.area());

    let t4 = Triangle::with_area(10, 100);
    assert_eq!(t4.height, 20, "Only one possible height value");
    println!("Cutom init triangle {:?}", t4);
}
