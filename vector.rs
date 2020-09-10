// vectors -> is an array on steroids
fn main(){
    // one way ↓, <type of vector>
    let vector: Vec<i32> = Vec::new();

    // the other way, tag "vec!"
    let mut vector1 = vec![1, 2, 3, 4];

    println!("{}", vector1[2]) // 3

    // push pushes a new element to the vector
    vector1.push(5);
    
    println!("{}", vector1) // 3
    
    // remove removes the element from the index given
    vector1.remove(1); // remove 2

    for n in vector1.iter() {
    println!("{}", n)
    }
    // we can use the `.enumerate()` and use (index, value)
}

