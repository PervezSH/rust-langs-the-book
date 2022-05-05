use std::error;
use std::fs::File;
use std::io::ErrorKind; // let us match on type of error we get
fn main() {
    /* Handling Error using using a Result Enum */
    let f = File::open("Hello.txt");
    // handling succes case and the error case
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            // create a file with the given name, if it doesn't exist
            ErrorKind::NotFound => match File::create("Hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file 🚫 {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file 🚫 {:?}", other_error)
            }
        },
    };

    /* Writing the exact using closures */
    let f1 = File::open("Hello1.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("Hello1.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file 🚫 {:?}", error);
            })
        } else {
            panic!("Problem opening the file 🚫 {:?}", error);
        }
    });

    /* Usefull functions on Result Enum */
    // instead of writing match expression, we can simply call unwrap()
    let f3 = File::open("Hello3.txt").unwrap(); // in error case, it will panic

    // also have expect(), to specify error msg
    let f4 = File::open("Hello4.txt").expect("Failed to open file 🤕");
}
