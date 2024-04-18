pub fn run(){

    let values=[1,2,3,4];
//iter() gives error i dont know 
    for value in values.iter(){
        println!("{}",value);
    }

    let iter = values.iter();
    //sum of all array
    let sum1 : i32=iter.sum();
    println!("{}",sum1);
    //iterators cannot be doubled consumend
    //cannot use same itrator variable twice 
    // let sum2 : i32=iter.sum(); => error
    //to solve :
    let iter2 =values.iter();
    let sum2:i32= iter2.sum();
    println!("{}",sum2);

    //mapping iteractor and collecting iterarors
    let sign: Vec<i32>= values.iter().map(|v|v*2).collect();
    println!("{:?}",sign);

    let vec =vec!["jumana","shimo","himo","mimio","jiji"];
    for v in vec.into_iter().filter(|name|name.len()==4){
        println!("{}",v);
    }
}