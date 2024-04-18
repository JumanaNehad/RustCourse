struct Person {
    //feilds
    name: String,
    age: u8,
}

// fn create_person(name: String, age: u8) {
//     let person = Person {
//         //they are the same
//         //hanshel wahda law homa same name
//         age: age,
//         name,
//     };
// }

//struct tuple for a 3d point that doesnt have feilds
//parantisis with no {} if there is no feilds
//plus tuple is in ()
#[derive(Debug)]
struct Point(f64, f64, f64);

//implementaions for struct
//lazem yekono nafs esm el struct
impl Point {
    //here i can create functions
    fn describe(&self) {
        //ma3rafsh eh lazmet el &
        println!("{} {} {}", self.0, self.1, self.2);
    }

    //fn return 0, doesnt need self here(non-associative function)
    fn zero()->Point{
        Point(0.0,0.0,0.0)
    }
}

impl Point {
      //this two functions are doning the same thing but the first retun new point and the second mutate the same point(changing the cuurent instance)
    //this function is not going to mutate Point , itsgoing to make new point with new values
    fn twice(&self) -> Point {
    
        Point(self.0 * 2.0, self.1 * 2.0, self.2 * 2.0)
    }

    fn make_twice(&mut self) {
        self.0 *= 2.0;
        self.1 *= 2.0;
        self.2 *= 2.0;
    }
}

pub fn run() {
    let person = Person {
        //they dont have to be in the same order as the struct
        age: 23,
        name: "jumana".to_string(),
    };

    println!("{} is {} years old", person.name, person.age);

    //update struct syntax
    let person2 = Person {
        name: "joudy".to_string(),
        //age: person.age,
        //similar to
        ..person //spread the rest of person here
    };
    println!("{} is {} years old", person2.name, person2.age);

    //initialize tuple
    let point = Point(0.0, 0.0, 0.0);
    //normal tuple
    // let point = (0.0,0.0,0.0);

    //calling
    println!("{} {} {}", point.0, point.1, point.2);

    let mut p = Point(1.0, 2.0, 3.0);
    p.describe();
    p.twice();
    p.make_twice();
    //or put discribe againg
    //beta3et el debug
    println!("{:?}", p);

    let point=Point::zero();
    point.describe();
}
