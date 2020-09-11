// 
fn main() {
    // replace
    {
        let string = String::from("Rust hahahaha!");
        println!("after: {}", string.replace("hahahaha", "great"));

    }

    // lines -> split the lines in the string to an iterator
    {
        let string = String ::from("the weather is\n bad\n very bad");
        for line in string.lines() {
            println!("- {} -", line);
        }
    }

    // split
    {
        let string = String ::from("go home mate");
        // returns an iterator so to convert it to a vector just use `.collect()`
        let tokens: Vec<&str> = string.split(" ").collect();

        println!("1 : {}", tokens[1])
    }

    // trim
    {
        let string = String ::from("go home\n mate           ");
        println!("{}", string);
        println!("{}", string.trim());
    }

    // chars
    {
        let string = String ::from("something for now");

        // string[4]; // for other languages this should give `t`
        // but in rust you have to do it a bit different
        // because it can or it can not exist on that position
        // returns at index 4
        match string.chars().nth(4) {
            some(c) => println!("index 4: {}", c),
            None => println!("none")
        }
        
    }
}