#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, compared_rect: &Rectangle) -> bool {
        let base_area : u32 = self.area();
        let compared_area : u32 = compared_rect.area();
        println!("Area of base rect: {}", base_area);
        println!("Area of compared rect: {}", compared_area);
        if base_area >= compared_area {
            return true;
        }
        false
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 100
    };
    let rect2 = Rectangle {
        width: 100,
        height: 100
    };
    println!("rect1 is {:#?}", rect1);
    println!("rect2 is {:#?}", rect2);
    println!("rect1 can hold rect2. This is {}", rect1.can_hold(&rect2));
}
