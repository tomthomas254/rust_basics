#[allow(unused_variables)]
fn main(){
    // error : unused varialble
    // // either prepend an underscore to the variable
    // // or use #[allow(unused_variables)] at the top of the file.    
    // let mut x:i32 = 1;//mutable variable x of type int

    // x += 1;

    // let x =5 
    // let x = 6 //this is called shadowing

    // let y: &str = "Hello";//immutable variable y of type string

    // //destructuring a tuple 
    // let (mut a, b) = (1,2);

    // //destructuring assignments

    // let (x, y);//same as let x ; let y

    // (x,..) = (3,2);//assign 3 to x, rest leave it
    // [..,y] = [1,2];//assign 2 to y , rest leave it
    // println!("{}{}",x,y);


    //for loop
    for i in 0..100000{
        println!("{}",i);
    }

    // for c in 'a' ..='z' {
    //     print!("{}",c);
    //     println!("-{}",c as u8);
    // }
}