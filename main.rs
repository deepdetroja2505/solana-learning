/*fn main(){
    println!("Hello, world!"); //macro
} */


// sum function example

/*fn main() {
    let ans:u32 =sum(9,2);
    
    println!("The sum is: {}", ans);
}

fn sum(a:u32, b:u32) -> u32 {
   return  a + b;
} */




/* ---------------DATA TYPES ------------------------- */






// boolean data type example

/*fn main() {


    let even:bool = is_even(4);
    println!("{}",even);
}

fn is_even(num:u32) -> bool {   // use only is_even✅ not isEven❌
    return num % 2 == 0;
}*/

//string data type example

/*fn main() {

    let greeting = String::from("hello world");
    println!("{}", greeting);
}*/


// char data type example

/*fn main() {

    let letter:char = 'A';
    println!("{}", letter);
}*/
// float data type example

/*fn main() {

    let pi:f32 = 3.14;
    println!("{}", pi);
}*/

// array data type example

/*fn main() {


    let numbers:[i32;5] = [1, 2, 3, 4, 5];
    println!("First number: {}", numbers[0]);
} */

//vector example

/*fn main() {

    let mut xs = vec![1, 2, 3];
    

    println!("{:?}", xs);   // to print whole vector

    print!("{}", xs.len());

    xs.push(4);
    
    print!("{}", xs.len());

    
} */