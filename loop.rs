fn main() {
    // using the loop key word
    looping();

    whiling();

    foring()
}

fn looping() {
    let mut n = 0;

    // every thing in the loop, will ...well loop until you give a condition that will stop the loop
    loop {
        n += 1;

        if n == 7 {
            // skip the 7
            continue
        }
        if n > 10 {
            // exit the loop
            break;
        }
        println!("the value of n is {}", n);
    }
}

fn whiling() {
    let mut n = 1;

    while n <= 50 {
        // n is a multiple to 5
        if n % 5 == 0 {
            println!("n is {}", n)
        }


        n += 1;
    }
}

fn foring() {
    // i is the value that will have the value
    // the 1..11 its an expression that generates a range 1 to 10 not including 11
    for i in 1..11 {
        println!("the number is {}", i)
    }
    
    println!("");

    // vec! is a vector (array)
    let animals = vec!["dog", "cat", "parrot"];

    // the function iter() -> iterator: this is for vectors this will prevent the ownership of the values
    // inside the vector to be moved to the for loop
    // example if you dont use iter() then it will not be possible to use
    // the vector in other places
    for a in animals.iter() {
        println!("the animal is is {}", a);
    }

    // the .enumerate() you can especify the index and the value
    // of the vector
    for (i, v) in animals.iter().enumerate() {
        println!("the index is {} the animal is is {}", i, v);
    }
}