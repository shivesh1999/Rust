use std::collections::HashMap;

fn main() {
    println!("OOOPS in Rust");
    let name_of_bird = String::from("lulu");
    let bird =Bird{name : name_of_bird, attack:5};
    bird.print_name();
    println!("{} {}",bird.can_fly(),bird.is_animal());
    let a:MyEnum=MyEnum::A;
    let b:MyEnum=MyEnum::B(10);
    let c:MyEnum=MyEnum::C{x:15,y:20};
    println!("{:?} {:?} {:?}",a,b,c);
    
    // vectors in Rust
    let mut vec: Vec<i32> = vec![1,2,3,4,5];
    println!("{} {} {}",vec.len(),vec[1],vec[2]);
    vec.push(6);//add by value
    vec.remove(1);//remove by index
    println!("{:?}",vec);

    //hashmaps in java 
    let mut map= HashMap::new();
    map.insert(0,"hi");
    map.insert(1,"Hello");
    println!("{:?}",map);
    // data is inserted like a qeue
    match map.get(&0) {
        Some(str)=>println!("{}",str),
        _ => println!("Doesnt exist in map")
    }
    map.remove(&0);
    println!("{:?}",map);

    // Used for wrapping values
    //Options
    let divide: Option<i32> = divide(4,2);
    println!("{:?}",divide);
    println!("{}",divide.unwrap());

}

//options
// None,to indicate failure or lack of value,and
// Some(value),atuple struct that wrapsavalue with type T.
fn divide(dividend:i32,divisor:i32)->Option<i32>{
    if dividend%divisor!=0{
          None
    }else{
          Some(dividend/divisor)
    }
}


// structures in rust
#[warn(dead_code)]
struct Bird{
    name: String,
    attack: u64
}
// for implementing methods to the struct 
impl Bird{
    fn print_name(&self){
        println!("{}", self.name);
    }
}

// inheritance is not supported in rust 
// interface is supported in rust 

// interface
trait Animal{
    fn can_fly(&self)->bool;
    fn is_animal(&self)->bool{
        return true;
    }
}

impl Animal for Bird{

    fn can_fly(&self)->bool{
        return true;
    }
}

//enums in RUST
#[derive(Debug)]
enum MyEnum{
    A,
    B(i32),
    C{x:i32,y:i32}
}