
use std::ops::Deref;
//import RC
use std::rc::Rc;
//import cell
use std::cell::Cell;
//import ref cell 
use std::cell::RefCell; 

//define our own box
// of type T : any value of any type
struct BoxedValue<T>{
    value:T
}

impl<T> BoxedValue <T>{
    fn new(value:T)->Self{
        //or {value} {value:value}
        BoxedValue{value}
    }
}

impl <T> Deref for BoxedValue<T> {
    //specify target
    type Target=T;

    fn deref(&self) -> &Self::Target {
&self.value    }
}


fn print_int(value:i32){
println!("{}",value);
}

struct Person{
    name:String,
    age: Cell<u8>
}


impl Person{
    //mafesh mut but it can mutate 
    fn increment_age(&self) -> u8{
//self.age+=1; cannot  use += in cell
 self.age.set(self.age.get()+1);
 self.age.get()
    }
}

pub fn run() {
    //box datatype stores in heap and it's a generic struct
    let age: Box<i32> = Box::new(22);
    //dereferencing
    let twice = *age * 2;
    println!("{}", twice);

    //instance of boxed value
    let age2= BoxedValue::new(22);
    //impl deref for boxvalue (mn gher deref dereferencing will fail)
    println!("{}",*age2);
    //deref a value
    let actual_age = age2.deref();
    println!("{}",actual_age);
    //point to the dereferenced value
    let actual_age2 = *age2;
    println!("{}",actual_age2);
    //the *ptr is a shorthand for *(ptr.deref())
    let other = *(age.deref());
    println!("{}",other);

    let value = BoxedValue::new(10);
    //pass this value to a function using ampersand
    //but it worked with me withh * astric 
    print_int(*value);

    let v= vec!["joudy".to_string(),"aly".to_string()];
    //create RC new instance
    let rc: Rc<Vec<String>> = Rc::new(v);
    //clone rc : create completly new one 
   // let rc2= rc.clone();

    //if rec drops it's value with here is the vector then weak reference will lose it's access to that vector
    let weak =Rc::downgrade(&rc);
    //to drop origninal rc //drops the value 
    drop(rc);

    //but will still have aceess in r2 (clone)
   // println!("RC2 {:?}",rc2);//will print aly , joudy

    //to access this weak we have to upgrade 
    //unwrap we will remove it if 
    //let test= weak.upgrade(); //datatype is an option or we expect that option here so we unwrapped it 
    let value2=weak.upgrade();//,unwrap()
    println!("{:?}",value2);//None as it dropped it's value 

    //another way as it's option
    match value2{
        Some(v)=> println!("{:?}",v),
        None=> println!("None")
    }

 //Cell
    let person =Person{
        name:"jumana".to_string(),
        //instance of new 
        age : Cell::new(23)
    };
   // let person_age= person.age.get(); //23
    let new_age=person.increment_age();//24
    let person_age= person.age.get(); //24
    println!("{}",new_age);
    println!("{}",person_age);

    //ref cell
    //creaion of ref cell
    let ref_cell= RefCell::new(vec![1,2,3,4]);

    //we cant have mut and immut borrows of same refcell will panic
    //borrow mut
    let mut mut_ref = ref_cell.borrow_mut();
    //push new value to vector 
    mut_ref.push(100);
    //borrow immutably + get length
    //let len = ref_cell.borrow().len();
//println!("{}",len);

}

