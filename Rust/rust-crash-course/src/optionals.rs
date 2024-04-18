pub fn run() {
    //creating optionals
    let value = Some("peter");
    let name = Option::<&str>::None; //like enum
    let name2: Option<&str> = None;

    //unwarapping options safly
    match name {
        Some(name) => println!("{}", name),
        None => println!("no name"),
    }

    //unwrapping unsafly
    // let unwrapped_name=name2.expect("noo name");
    //it will print if name=none only
    //  println!("{}",unwrapped_name);

    //force unwrap
    //panic if none and act normal if some
    // let unwrapped_name=value.unwrap();
    // println!("age is {}",unwrapped_name);

    //mutate
    let mut age = Some(10);
    match age.as_mut() {
        Some(age) => *age+=10,
        None => println!("h"),
    };
    //mn gher unwrap fi howar
    //badal msh fa match lazem unwrap
println!("age = {}",age.unwrap());

    //if intersted in one 
    if let Some(l) = age.as_mut() {
        println!("age is {}", l);
    }
    
    //unwarap multiple options
    let n1= Some(20);
    let n2= Some(20);
    let n3= Some(20);

    if let (Some(age1),Some(age2),Some(age3))= (n1,n2,n3){
        println!("{}",age1+age2+age3);
    }

    //put default value for option
    let k:Option<&str>=Some("sarah");
   // let def = k.unwrap_or("John");
   // println!("{}",def);

    //check if having a value
    if k.is_some(){
        println!("fi value");

    }
    else{
        println!("mafeesh value");
    }

    //default value ex: for i32 used if no value
    let w:Option<i32> = None;
  //  let w = w.unwrap_or_default();//0
   
   // println!("{}",w);

   //mapping
   
   let mapping = w.map(|x|x*2);
   println!("{}",mapping.unwrap_or_default());//to get default value when print 

}
