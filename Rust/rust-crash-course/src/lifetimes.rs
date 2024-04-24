//String can be returned but &str slice cant be returned
//lifetime operators in rust written before datatype=> 'static
//this mean that this string needs to live on for the entirty of the application lifetime
fn get_fullname() -> &'static str {
    "John doe"
}

//genet=ric lifetime annotations
//as long a and b are leaving I want the return in the memory
//wah have 2 lifetime specifiers <'a , 'b>
// a and b can have different name or letter
fn get_random_name<'l>(a: &'l str, b: &'l str) -> &'l str {
    //b  //if we put here b will gives error as it's return living as long a is living to solve that make them use the same specifiers name
    b
}

//lifetime in struct
//person has live time specifier
//name lives as long as the instance that we have created live
//can write &'static here but this is more preferred
struct Person<'a> {
    firstname: &'a str,
    lastname: &'a str,
}
//mn gher dol keda hayedi error
impl<'a> Person<'a> {
    fn first_char_firstname(&self) -> &str {
        &self.firstname[0..1]
    }
    //as it's fullname use string
    //output is not a reference no lifetime in here 
    fn fullname(&self) -> String {
        format!("{} {}",self.firstname,self.lastname)
    }
}

//no error without specifier why???
//if you have reference paramter the result also is gonna have the exact same lifetime
//has hidden specifier
fn get_name(name: &str) -> &str {
    name
}

//enum lifetime
enum Animal<'a> {
    Dog{name:&'a str}, //mn gher specifier will give an error
}

pub fn run() {
    let fullname = get_fullname();
    println!("{}", fullname);

    let rand = get_random_name("J", "D");
    println!("{}", rand);

    let j = Person {
        firstname: "juja",
        lastname: "k",
    };
    println!("{}", j.firstname);

    let name = get_name("joudy");
    println!("{}", name);

    let char =j.first_char_firstname();
    println!("{}",char);

    let fullname=j.fullname();
    print!("{}",fullname);



}
