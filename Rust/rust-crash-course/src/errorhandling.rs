
fn last_name() -> Result<String, String> {
    Ok("Joury".to_string())
}
                            //hanshel () and put String(to return error string)
fn first_name() -> Result<String,String> {
  //  Ok("kim".to_string())
   Err("This is an error".to_string())
}
//if one of these functions has error, we return error in this func
fn full_name() -> Result<String,String>{
    let firstname= first_name()?;
    let lastname= last_name()?;
    Ok(format!("{} {}",firstname,lastname))
}

//function return result //I cant be str => lifetime error
fn get_user_name() -> Result<String, ()> {
    Ok("bella".to_string())
    //Err(())
}


pub fn run() {
    //i have a result type which can either be string slice or error
    //we have to put error in a box which is reference to memory in the heap
    //error:Error is a trait
    //actual result =>&str //Result = Ok or Err
    let value: Result<&str, Box<dyn std::error::Error>> = Ok("hello");

    //matching error
    match value {
        Ok(value) => println!("{}", value),
        Err(error) => println!("{}", error),
    }

    //void : basically nth
    let value2: Result<&str, ()> = Err(());

    match value2 {
        Ok(value) => println!("{}", value),
        //takes nth
        Err(_) => println!("some error occurred"),
    }

    //expecting a value , expect msg printed if there's an error
    let name = get_user_name().expect("I was expecting a name");
    println!("{}", name);

    //we can expect an error
    //printed if there's no error
    //let user_name=get_user_name().expect_err("I didn't expect name");

    //check function is returning ok or error
    let is_ok = get_user_name().is_ok();
    let is_error = get_user_name().is_err();
    println!("{} {}", is_ok, is_error);

    //Early exit from result errors
    let fullname= full_name();

    // match fullname {
    //     Ok(name) => println!("{}",name),
    //     Err(_)=>println!("Error!")
    // }

    //if you want to map ok in result
    //I cant use that after match 
    //unwrap_or_default() => if it's an err by default use length of string =0
    let length = fullname.map(|s|s.len()).unwrap_or_default();
    println!("{}",length);

    let fullname2= full_name();
    //map err in result //maynf3sh a3mel mapping le nafs varible maretten
        let error_length= fullname2.map(|e|e.len());
        println!("{:?}",error_length); //this should give me number of msg error but it print the msg and I dont know why
}
