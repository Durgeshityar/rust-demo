fn main() {

// example 1 works dealing with stack memory
let x:u8 =5;
process_integer(5);
println!("The values of x in main in  {}", x);

//example 2- > dynamic memory heap
// wroks 
let y:String = String::from("Hello string"); // y owner
println!("The value of  y in main is {}", y); // access y 
process_string(y); // new owner outside_y


// wont work -> 
let z:String = String::from("Hello string 2"); // z owner
process_string_z(z); // transfer ownership to outside_z 
//println!("The value of  z in main is {}", z); // z is invalid


// dealing with unexpected behaviour of ownership
let  s1 = String::from("OWNERHIP"); // s1 owner
let (s2,len) = calculate_len(s1); // ownership transfer to s -> returns ownership to s2
println!("The lenght of s1 is  {} and value of s1(now s2) is  {}", len, s2);


//efficent way -> clone
let s3 = String::from("ownership 2");
let len2 = calculate_len2(s3.clone());
println!("The lenght of s3 is  {} and value of s3 is  {}", len2, s3);

// more efficient way -> refrence
let  s4 = String::from("ownership 3");
let len3 = calculate_len3(&s4); // borrow operation -> borrowed values are immutable
println!("The lenght of s4 is  {} and value of s4 is  {}", len3,s4);

let  mut s5 = String::from("ownership 4");
 append_string(&mut s5); // mutable borrow operation -> borrowed values are
println!("The value of s5 is {}", s5);


// There are some rules for refrences too. For read only it wont matter
// this is correct because there is no conflict -> w1 created used  and dropped before w2 -> but if w1 is accesed after  w2 then it will be an error, creates conflict
// similar conflict can also occur in read and write 
//conclusion -> dont perferform multipe read and write  operations at the same time
let  mut s6 = String::from("ownership 5");

let w1 = &mut s6;
w1.push_str("w1 incoming");
println!("w1={}",w1);

let w2 = &mut s6;
w2.push_str("w2 incoming");
println!("w2={}",w2); // accesing  w1 after w2 is created is an error



// refrencing, derefrencing and auto derefrencing




}


fn calculate_len (s:String) -> (String, usize){
let length:usize = s.len();
return (s,length); // return ownership transfer,5
}

fn calculate_len2 (s:String) -> usize{
let length:usize = s.len();
return length; // 5
}

fn calculate_len3 (s:&String) -> usize{
// borrowed values are immutable
let length:usize = s.len();
return length; // 5
}

fn append_string (s:&mut String) {
s.push_str("appende value");
}

fn process_integer(x:u8) {
    println!("The  value of x in process_integer is {}", x);
}

fn   process_string(outside_y:String) {
    println!("The value of x in process_integer is {}", outside_y);
}

fn process_string_z(outside_z:String) {
    println!("The value of x in process_integer is {}", outside_z);
}