use std::f32::consts::PI;

#[derive(PartialEq)]
enum AnimalType {
    //cases has to be in capital letter
    Dog,
    Cat,
    Rabbit,
}

enum Shape {
    //circle has dol     //center has tuple
    Circle { radius: f64, center: (f64, f64) },
    Rectangle { width: f64, height: f64 },
}

enum Shape2 {
    //unnamed associative value
    //struct
    Rectangle(f32, f32, Size),
    Circle(f32, f32, f32),
}

struct Size {
    width: f32,
    height: f32,
}

impl Shape2 {
    fn area(&self) -> f32 {
        match self {
            Shape2::Rectangle(x, y, size) => size.width * size.height,
            Shape2::Circle(x, y, radius) => PI * radius * radius,
        }
    }
}

enum Pet {
    Cat { name: String },
    Dog { name: String },
}

pub fn run() {
    //to create instance
    let bella = AnimalType::Dog;
    if bella == AnimalType::Dog {
        println!("Bella is a dog!");
    }

    match bella {
        AnimalType::Dog => println!("whoof"),
        // AnimalType::Cat => println!("meow"),
        // AnimalType::Rabbit => println!("hoot")
        //anything that is not already handled
        _ => println!("Smth else"),
    }

    let rectangle = Shape::Rectangle {
        width: 3.0,
        height: 6.9,
    };

    //use if let
    if let Shape::Rectangle { width, height } = rectangle {
        println!("{} ,{}", width, height);
    }

    //and we can use match
    match rectangle {
        Shape::Rectangle { width, height } => println!("{},{}", width, height),
        _ => println!("not a rectange"),
    }
    // create instance
    let rectangle2 = Shape2::Rectangle(
        3.1,
        3.2,
        Size {
            width: 3.1,
            height: 4.9,
        },
    );

    //if let
    //no numbers here so we write x , y
    if let Shape2::Rectangle(x, y, Size { width, height }) = rectangle2 {
        println!("{},{},{},{}", x, y, width, height);
    }

    match rectangle2 {
        Shape2::Rectangle(x, y, Size { width, height }) => println!("rectangle 2"),
        _ => println!("not rectangle"),
    }

    let area = rectangle2.area();
    println!("{}", area);

    let fluffy = Pet::Cat {
        name: "Fluffy".to_string(),
    };
    //how to know fluufy return any name the two have a name
    let name = match fluffy {
        //I can return number or anything without typing return
        Pet::Cat { name } => name,
        Pet::Dog { name } => name,
    };
    println!("{}", name);
}
