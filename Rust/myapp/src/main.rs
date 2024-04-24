#![deny(clippy::all)]
//error in intuils so we have to go to cargo tomol el outside  one
//path to function 
//package::Crates::module 
use intutils::subtraction::subtract;
use intutils::addition::add;
fn main() {
    let added= add(1,2);
    let sub = subtract(5,2);
    println!("{}",added);
    println!("{}",sub);
}
