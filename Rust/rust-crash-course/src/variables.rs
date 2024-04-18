//add clippy to main we need its help in writing the code in this file

const MY_AGE: u8 = 22; //constant must have a default value
pub fn run() {
    //datatype wirtten automatically from rust anaylser or mn hga ana menazelaha which is rust analyser
    let name = "Jumana";
    //  name="nehad";
    println!("my name is {}", name);

    //the exact same thing
    let age: u8 = 20;
    let age1 = 20u8;
    println!("{} {}", age, age1);

    //operatores
    let num1 = 2.4;
    let num2 = 5.7;
    let num3 = 8.0;
    //it cant make the some of all it will add num1 and num2 and the result will be added to num3
    let total = num1 + num2 + num3;
    println!("{}", total);

    //variable shadwoing
    //let data ="joo"; //cooment 3ashan kanet medaya waring
    let data = "hai";
    {
        //  let data = data.to_string(); //(w)
    }
    println!("{}", data); //hai

    println!("{}", MY_AGE);

    //The variable can contain value of const
    let my_age = MY_AGE;
    println!("{}", my_age);

    //tuple   //when I want to change datatype shange it from tuple values
    let myy_data = (22u8, "juaman");
    //assign variables to tuple values //pack the data
    let (ago, namo) = myy_data;
    println!("{} {}", ago, namo);
}
