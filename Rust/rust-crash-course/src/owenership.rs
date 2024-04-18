//immutable function
//this function takes name which is variable refernce 
//name which is a pointer to a string
fn greet(name: &String) {
    println!("Function {}", name);
}

//mutate fun
fn empty_string(value:&mut String){
value.clear(); //to change this we should put mut as this is a borrowed variable cant change it 
}
             //we cant return string refernece 
// fn get_name () -> String {
// "john".to_string()
// }

pub fn run() {
    //string msh byt3mel gher keda bydei errors 3ashan byshofo str 3ady
    let name1: String = String::from("jumana");
    let name2 = name1;
    //if i print name1 and name2 it is going to give error
    //this concept named moving it is done using traits
    println!("{}", name2);

    //but in case of intergers it will work normally
    let age1 = 10;
    let age2 = age1;
    println!("{}", age1);
    println!("{}", age2);

    //refernce
    let mut n1: String = String::from("Reference");
    //string reference
    let n2 = &n1;
    println!("{}", n1);
    println!("{}", n2);
    //will not work with n1 as it's not string references 
    greet(n2);
    //unless : putting refernce before variable
    greet(&n1);
    empty_string(&mut n1);
    
//two mutable variable in referneces to prevent data races
//let mut cake = String::from("mutable");
//let mut cake1 = &mut cake;
//let mut cake3 = &mut cake; //this will gives error
//empty_string(&mut cake1);

    //block of code
    {
        let name = "blockname".to_string();
        println!("{}", name);
    }
    //i cant access name outside this block 3ashan byt3melaha de alocationg fel mem
}
