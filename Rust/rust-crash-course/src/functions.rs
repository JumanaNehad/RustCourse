//simple functions
fn say_hello() -> String {
    String::from("Hello , world!!") //automatically return from this fun
}
//doesnt return anything
fn say_hello2() {
    //unit type
    let message = String::from("Hello , world2!!");
    println!("{}", message);
}

//with argumant
fn say_hello3(message: &str) {
    println!("{}", message);
}

fn say_hello_to_person(person: String) -> String {
    format!("hello , {}", person)
}

//put another fun as an argumant
fn process_name(name: &str, callback: fn(&str) -> ()) {
    callback(name);
}

pub fn run() {
    let message = say_hello();
    println!("{}", message);

    say_hello2();

    say_hello3("Hello , wrold3");
    let hello = say_hello_to_person(String::from("jumana"));
    println!("{}", hello);

    //inline function //i dont have to specify the return type
    let inline_fun = |name: &str| format!("Hello, {}", name);
    println!("{}", inline_fun("inline fun"));

    let full_name = |first_name: &str, last_name: &str| format!("{} {}", first_name, last_name);
    println!("{}", full_name("juja", "nehad"));

    //incline with multiple statmenets
    let ask_for_age = || {
        //ask the user for age
        //calculate how old in 10 years
        // if i put 10 without semi column = it will understand that you return that and change fn structure
        10
    };
    println!("{}", ask_for_age());

    //point to incline func
    let multiply_by_2 = |x: i32| x * 2;
    let ptr = multiply_by_2;
    let result = ptr(10);
    println!("{} ", result);
}
