use std::process::Command;

fn main() {
    // create an instance of Command
    // new("the name of the executable name, example `sgo run main.go`")
    let mut cmd = Command::new("echo");
    cmd.arg("something to echo!!!");

    // exec the command
    match cmd.output() {
        Ok(o) => {
            // => the output of the command executed, will be bytes and not a string
            unsafe {
                // the `form_utf8_unchecked` is unsafe because it does not check if the bytes
                // are a valid utf8
                println!("output: {}", String::from_utf8_unchecked(o.stdout));
            }

        },
        Err(e) => println!("error {}", e)
    }

}
