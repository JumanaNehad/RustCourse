
fn get_tuple_values() -> (String, String, i32) {
    ("jojo".to_string(), "nono".to_string(), 23)
}

pub fn run() {
    //pack
    let values = ("jumana", "nehad", 23);
    //unpack
    let firstname = values.0;
    let lastname = values.1;
    let age = values.2;
    //or unpack in a group
    let (first, last, ago) = values;

    //ignoring all exept age
    let (_, _, age1) = values;

    let (hello, _, _) = get_tuple_values();
    println!(
        "{},{},{},{},{},{},{},{}",
        firstname, lastname, age, age1, hello, first, last, ago
    );

    //fixed size
    //datatypes and i an specify number of items
    let val = [1, 2, 3, 4];
    //to iterate
    for value in val.iter() {
        println!("{}", value);
    }
    let bella_age = &val[3];
    println!("bella's age is {}", bella_age);

    //to get length
    let length = val.len();
    println!("length {}", length);

    //double by mapping
    let doubled = val.iter().map(|x| x * 2);
    for x in doubled {
        println!("{}", x);
    }

    //not fixed
    let mut numbers=vec![1,2,3,4];
    numbers.push(7);
    //let four = numbers.pop();
    println!("values are {:?}",numbers);
    //adding new values to the end of the vector
    numbers.extend_from_slice(&[4,5,6]);
    println!("values are {:?}",numbers);
    //clear all values
    numbers.clear();
    println!("values are {:?}",numbers);

    //move from one vector to another
    let mut vector2=vec![7,8,9];
    numbers.append(&mut vector2);
    println!("vector moved values{:?}",vector2);
    println!("complete vector{:?}",numbers);

    //test if vector contains a value
   if numbers.contains(&3){
        println!("yes");
    } else{
        println!("no");
    }

    //check if empty
    if numbers.is_empty(){
        println!("empty vec");
    }
}
