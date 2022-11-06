fn main() {
    println!("Hello, world!");
    let a=10;
    println!("{}",a);
    // variable in rust in immutable 
    // more making variable mutuable you have to add keyword mut infront of the variable
    // let b:u8=10;
    // let float:f32=3.14;
    // let letter ="abcd";
    // let is_true:bool=true;

    //Arrays
    let arr:[u8;3]=[1,2,3];
    let other_array:[u8;5]=[100;5];
    println!("{} first element of arr, {} length of other_array",arr[0],other_array.len());
    println!("{:?}",other_array);// prints the strucyute of the array

    //tupples
    let tuple1:(u8,bool,f32)=(4,true,2.1);
    let tuple2=(1,2,3,4,5);
    println!("tuple1 {:?}, tuple2 {:?}",tuple1,tuple2);
    //destructuring tuple
    let (a,b,c)=tuple1;
    println!("first {}, second {}, third {}",a,b,c);
    
    //functions
    let even:bool=check_even(5);
    println!("{}",even);

    //arrrays and slices 
    let arr=[1,2,3];
    //slices are basics subarrays
    let slice=&arr[1 .. 3];
    // here first value is inclusive and
    //last value is exclusive 
    // we dont know the length 
    // & is added for refrencing 
    // add _ infront of varables to get rid of the warnings
    println!("{:?}, slice {:?}",arr,slice);


    //Strings in Rust
    let str:&str="hello world";
    let mut text:String=String::from("Hello World");
    let slice=&text[.. 5];
    println!("{:?}",slice);
    text.push('!');
    text.push_str(" shivesh");
    text=text.replace("Hello","Bye");
    println!("{}",text);


    //if-else
    if a > 10{
        println!("a is grater than 10");
    }
    else if a < 10{
        println!("a is less than 10");
    }
    else{
        println!("a is equal to 10");
    }

    //for loop
    for i in 0..6{
        println!("{}", i);
    }

    //while loop
    let mut j=5;
    while j>=0{
        println!("{}",j);
        j=j-1;
    }

    //match
    // work like switch case statement
    let k=3;
    match k {
        0 => println!("0"),
        1 | 2 => println!("1 or 2"),
        3..=4 => println!("3,4"),
        _ =>  println!("default")
    }
}
fn check_even(num:u8)->bool{
    let remainder=num%2;
    return remainder==0;
}
