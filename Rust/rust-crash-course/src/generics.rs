// +
use std::ops::AddAssign;

//point could be int or float
//so will put T : means any type
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn move_offset(&mut self, x: T, y: T)
    where
        T: AddAssign,
    {
        //T type + and = operaters gives error as it doesnt know what are the value that are going to work with
        //to solve import AddAssign
        self.x += x;
        self.y += y;
    }
}

//they are like ready made traits
impl<T: AddAssign> AddAssign for Point<T> {
    //other: copy of self
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl<T: PartialEq> PartialEq for Point<T> {
    //made by LAMPVS
    // other another instance of self
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

trait Canrun {
    fn run(&self);
}

trait CanWalk {
    fn walk(&self);
}
//as long as vector items can run the entire vector can run
impl<T: Canrun> Canrun for Vec<T> {
    fn run(&self) {
        for item in self {
            item.run();
        }
    }
}

impl<T: CanWalk> CanWalk for Vec<T> {
    fn walk(&self) {
        for item in self {
            item.walk();
        }
    }
}

struct Person2 {
    name: String,
}

impl CanWalk for Person2{
    fn walk(&self){
        println!("{} is walking ",self.name);
    }
}

impl Canrun for Person2{
    fn run(&self){
        println!("{} is running ",self.name);
    }
}

struct Elephant{
    name:String
}

impl CanWalk for Elephant {
    fn walk(&self){
        println!("{} is walking ",self.name);
    }
}

pub fn run() {
    let mut p1 = Point { x: 2.0, y: 1.0 };
    //if we add float instead of int will get error
    //lehad ma hatena T
    let p2 = Point { x: 3.0, y: 4.0 };
    //let p3=Point{x:"jj",y:"kk"};

    p1.move_offset(3.0, 4.0);
    println!("{:?}", p1);

    //move p1 by the value of p2
    //they have to be of the same type
    //working because I implement addAssign in entire point
    //mn gher trait addassighn will give error
    // p1 += p2;
    // println!("{:?}", p1);

    //this gives an error without PartialEq trait
    //3ashan type T
    if p1 == p2 {
        println!("p1 and p2 are equal");
    } else {
        println!("p1 and p2 are not equal");
    }

    let people = vec![
        Person2{
            name:"jessi".to_string(),
        }  ,
        Person2{
            name:"jane".to_string(),
        },
        Person2{
            name:"joe".to_string(),
        }
    ];
    people.run();
    people.walk();
}
