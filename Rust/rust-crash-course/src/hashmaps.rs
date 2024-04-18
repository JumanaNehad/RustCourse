//if have to include hashmap in my file
use std::collections::HashMap;

pub fn run(){
//instance of hashmap
let mut values: HashMap<&str, &str>=HashMap::new();
//use insert function to insert values
values.insert("m", "j");
println!("{:?}",values);

//checking existence of keys in hashmap
if values.contains_key("name"){
    println!("name exists");
}
else {
    println!("no name");
}

//to remove key and a values 
           //key
values.remove("m");
println!("{:?}",values);

values.insert("juja", "dola");
//this is quite unsafe if you misspel the key
let bar = values["juja"];//key
println!("bar is {}",bar);//dola

//safely reading values 
let bar = values.get("juja");
//use get function 
match bar{
    Some(value)=>println!("{}",value),
    None => println!("not found")
}

//iter hashmap
for (&k , &v) in &values{
    println!("{} {}",k,v);
}

//for reterieving the hashmap but it is rearly used 
//let entry =values.entry("juja");

//insert in key absence
values.insert("husband", "john");
values.entry("wifi").or_insert("jana");
println!("{:?}",values);
}