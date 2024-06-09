struct Rect {
    width: u32,
    height: u32,
}

//function that uses rect as variable.
fn area(rect: &Rect) -> u32 {
    rect.width * rect.height
}

impl Rect {
    //method definition starts with self
    fn area1(self: &Self) -> u32 {
        self.width * self.height
    }

    //It can be shorten as below.
    fn area2(&self) -> u32 {
        self.width * self.height
    }
}

//Implementation of method and associated functions are not need to be in a single block.
//It can be divided into several blocks
impl Rect {
    //Method name can be the same with a field.
    fn width(&self) -> u32 {
        self.width
    }

    //Method can receive the arguments
    fn can_hold(&self, rect: &Rect) -> bool {
        self.width > rect.width && self.height > rect.height
    }

    //Associated functions are functions that doesn't require self.
    fn square(size: u32) -> Rect {
        Rect {
            width: size,
            height: size,
        }
    }
}

//Rust program starts with main function
//Function starts with fn
//Function parameter exists in parenthesis.
//Block defined with {} brackets.
fn main() {
    let rect = Rect {
        width: 20,
        height: 30,
    };
    println!("The area of rect is {0}", area(&rect));
    //The area of rect is 600
    println!("The area of rect is {0}", rect.area1());
    //The area of rect is 600
    println!("The area of rect is {0}", rect.area2());
    //The area of rect is 600
    println!("The width of rect is {0}", rect.width());
    //The area of rect is 20
    println!("The width of rect is {0}", rect.width);
    //The area of rect is 20

    let rect1 = Rect {
        width: 30,
        height: 50,
    };

    let rect2 = Rect {
        width: 10,
        height: 40,
    };

    let rect3 = Rect {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    // Can rect1 hold rect2? true
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    // Can rect1 hold rect3? false

    //Associated function doesn't use self so it need to be called from struct itself.
    let s = Rect::square(5);
    println!("Square width: {} area: {}.", s.area1(), s.width());
    //Square width: 25 area: 5.
}
