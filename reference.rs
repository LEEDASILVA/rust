// another way to refere to variables
// for example: someone thats called Julia and you call Jul, you are
// refering to the same person but in different ways
fn main() {
    let mut x = 10;
    let xr = &x; // get a reference to x

    println!("x is {} and xr is {}", x, xr);

    // xr += 1; // does not work!
    
    let a = &mut x; // mutable reference

    *a += 1; // will work!

    println!("a is {}", a);
    
    // this will give error, you can define a mutable reference to x and 
    // at the same time borrow it, so a borrows x!
    // println!("x is {}", x);

    // to do this you can
    // {
    //     let a = &mut x; // mutable reference
    //     *a += 1; // will work!
    // }

    // x will be 11!
    // println!("x is {}", x);
}