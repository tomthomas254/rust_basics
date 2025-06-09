fn main(){
    println!("Hello, World");
    hello_world();
    tell_height(182);
    id("John Doe", 23);

    let x = {
        let price = 5;
        let qty = 10;
         price * qty //it's same as return price * qty
    };
    println!("Result : {}", x);

    let y = add(7, 8);
    println!("Result : {}", y);
}

fn add(a:i32, b:i32)->i32{
    a + b
}

fn hello_world(){
    println!("Hello, Rust!")
}

fn tell_height(height:i32){
    println!("Given height {}cm", height);
}

fn id(name:&str, age:i32){
    println!("My name is {}, age is {}",name, age);
}