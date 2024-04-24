use std::fmt; //fmt for display trait

#[derive(Debug)]
struct Person {
    firstname: String,
    lastname: String,
    age: u8,
}

//display trait
impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} (Age {})", self.firstname, self.lastname, self.age)
    }
    //vs can write this automatically
}

//crete my trait
trait HasFullName {
    fn fullname(&self) -> String;
}

//implement your trait in person
impl HasFullName for Person {
    fn fullname(&self) -> String {
        format!("{} {}", self.firstname, self.lastname)
    }
}

trait CanRun {
    fn run(&self);
}

impl CanRun for Person {
    fn run(&self) {
        //todo
    }
}

//traits as parameters
// HasFullName instead of writing type Person to use functions for more than one struct
fn print_fullname(value: &impl HasFullName) {
    println!("{}", value.fullname());
}

//trait bound syntex
//same as the above fn but in better way
//confermance to multiple traits by + sign we can put new trait
fn print_details<T: HasFullName + CanRun>(value: &T) {
    println!("{}", value.fullname());
    value.run();
}

//trait parameter usinng where
//same as above
fn print_details2<T>(value: &T)
where
    T: HasFullName + CanRun,
{
    println!("{}", value.fullname());
    value.run();
}

//triat with new function
trait CanInitializeWithFullName {
    fn new(fullname: &str) -> Self;
}

impl CanInitializeWithFullName for Person {
    //takes the fullname then split it
    fn new(fullname: &str) -> Self {
        //spliy using space
        //collect turning map of string slices into a vec of string slices
        let parts: Vec<&str> = fullname.split(' ').collect();
        //create persom
        Person {
            firstname: parts[0].to_string(),
            lastname: parts[1].to_string(),
            age: 30,
        }
    }
}

trait HasName {
    fn first_name(&self) -> &str;
    fn last_name(&self) -> &str;
}

impl HasName for Person {
    fn first_name(&self) -> &str{
        &self.firstname
    }

    fn last_name(&self) -> &str{
        &self.lastname
    }
}

//define hasfullname
//or // trait HasFullName2: HasName
trait HasFullName2
where
    Self: HasName,
{
    fn fullname2(&self) -> String ;
  //  {
        //I can erase this and implemented later
        //format!("{} {}", self.first_name(), self.last_name())
 //   }
}

//implement hasfullname2 on HasName
//T any obj , so hasfullnam2 can conform to hasname
impl <T> HasFullName2 for T where T:HasName {
    fn fullname2(&self) -> String {
        format!("{} {}", self.first_name(), self.last_name())
    }
}

pub fn run() {
    //create an instance
    let person = Person {
        firstname: "ALy".to_string(),
        lastname: "nino".to_string(),
        age: 18,
    };

    //error without debug trait
    //instead can write person.firstname
    //but can can that without debug
    println!("{:?}", person);

    //new I can create new instance of Person using new function
    let person2 = Person::new("jumana Nehad");
    println!("{} {} {}", person2.firstname, person2.lastname, person2.age);

    //without display trait = error
    println!("{}", person2);

    //print trait paramter
    print_fullname(&person);
    print_details(&person2);
    print_details2(&person);

    //print person full name
    let full_name = person2.fullname2();
    println!("{}",full_name);
}
