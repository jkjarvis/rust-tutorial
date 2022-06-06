struct Rectangle{
    width: u32,
    height: u32
}

impl Rectangle{
    fn area(&self) -> u32{
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle)->bool{
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32)->Rectangle{ //associated function
        Rectangle{
            width: size,
            height: size
        }
    }
}

fn main() {
    // let width1 = 30;
    // let height1 = 50;

    // println!("The area of rect is {} sq. pixels",area(width1, height1));

    // let rect = (30,50);

    // println!("THe area of rect is {} sq. pixels",area(rect));

    let Rect = Rectangle{width:30, height:50};
    // println!("Area is {}", area(&Rect));

    println!("Area of rect is {}",Rect.area());

    let sq = Rectangle::square(3);
}

// fn area(width: u32, height: u32)->u32{
//     width*height
// }

// fn area(dimension: (u32,u32))->u32{
//     dimension.0*dimension.1
// }

fn area(Rect: &Rectangle)->u32{
    Rect.width*Rect.height
}