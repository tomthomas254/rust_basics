// Compound Data types
//  arrays, tuples, slices and strings(slice string)

// Arrays: fixed size collection of elements of the same type of
//          the indegeneous type.
//          eg, let array_name:[type; size_of_array] = [elements]
//          eg, let number:[i32; 5] = [1,2,3,4,5]

fn main(){
    let number:[i32; 5] = [1,2,3,4,5];
    let a = [3;5]; // The array named a will contain 5 elements that will all be set to the value 3 initially. This is the same as writing let a = [3, 3, 3, 3, 3]; but in a more concise way. 
    // println!("Numbers array: {}", number) - will throw error: `[i32; 5]` cannot be formatted with the default formatter
    println!("Numbers array: {:?}", number);//output - Numbers array: [1, 2, 3, 4, 5]
    println!("Numbers array: {:#?}", number);//output -Numbers array: [
                                                //     1,
                                                //     2,
                                                //     3,
                                                //     4,
                                                //     5,
                                                // ] 
    
    let fruits : [&str; 3] = ["apple", "Banana", "Orange"];
    println!("Fruits: {:?}",fruits);
    println!("Fruits 1: {}",fruits[0]);
    println!("Fruits 2: {}",fruits[1]);
    println!("Fruits 3: {}",fruits[2]);

    // Tuples

    let person:(String, i32, bool) = ("Alice".to_string(), 30, false);
    let person1:(String, i32, bool,[i32;3]) = ("Alice".to_string(), 30, false, [1,2,3]);
    println!("Person: {:?}, {:?}, {}",person, person1,person.0);

    // Slices

    // Strings vs String slices(&str)
  
    
   
}