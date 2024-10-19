// imports the env module from standard library. Involves interaction with enviroment variables and obtain the current directory
use std::env;
// Imports the file system module, involves file maniputlation such as read and write.
use std::fs;
// import io for relavant operations.
// imports write to flush output buffers ensuring printed messages appear immediatley.
use std::io::{self, Write};
// imports the Path type
// represents a file path in the file system, used for manipulating and handling paths.
use std::path::Path;

// import file from fs, to create files
use std::fs::File;
use std::path::PathBuf;

// main function, entry point.
// Result is error pattern matching.
fn main() -> io::Result<()>{
    // make a immutable var called "current_dir"
    // use standard library env method current_dir() to store current directory
    // Current_dir has Result error pattern matching.
    // because of the "?" automatic handling of errors and will immediatly return the error is something goes wrong. For debugging.
    let mut current_dir = env::current_dir()?;
    // println! macro for generating print-line code, setting curren_dir inside
    // .display() formats the "path" as a string so it can be properly displayed.
    println!("Current Directory: {:?}", current_dir);
    // called list_files method
    // pass var "current_dir" as reference (immutable)
    list_files(&current_dir)?;

    // call loop to continuosly execute user commands until manual exit
    loop {
        // create mutable variable "command" and set it as an empty String
        // using String as it is mutable, as there is a trim later on.
        let mut command = String::new();
        // Asks user to enter command using print macro
        print!("Enter a command (list: create (file-name): Change (directory) delete: exit: ): ");
        // flushes the output buffer to ensure that the prompt appears immediatly
        // "?" propagates errors to return immediatly
        io::stdout().flush()?;
        // reads a line of input from user and stores it inside the command var, again error propagation
        io::stdin().read_line(&mut command)?;
        // setting the old command var into an immutable var after trimming
        let command = command.trim();
        if command == "exit" {
            break; // break the loop and end the program
        } else if command.starts_with("delete ") { // starts with to better get command from user
            let filename = &command[7..]; // slice to remove "delete" from command
            delete_file(&current_dir, filename)?; // call delete_file method to delete chosen file
            
        } else if command.starts_with("create") {
            let filename = String::new();
            let mut filename = &command[..7].to_string();
            create_file(&current_dir, filename)?;
            
        } else if command.starts_with("change") {
            let dir = &command[7..].to_string();
            change_directory(&dir)?;
            current_dir = env::current_dir()?;
        }else {
            println!("Unknown Command") // if user entered in command wrong
        }
         
    }


    println!("Application Closing!");
    Ok(())
}

fn list_files(path: &Path) -> io::Result<()>{
    let entries = fs::read_dir(path)?; // error propagation. Sets "entries" to read files in path.
    for entry in entries {
        let entry = entry?; // unwrap entry and handle potential errors
        let filename = entry.file_name(); // get current file name for printing
        println!("{}", filename.to_string_lossy()); // print filename, handle potentially non UTF-8 characters gracefully
    }
    Ok(()) // returns affirmation function has succeded to Result in main.
}

fn delete_file(path: &Path, filename: &str) -> io::Result<()>{
    let file_path = path.join(filename); // make complete file path
    if file_path.exists(){
        fs::remove_file(file_path)?;
        println!("File path: {}, has been deleted!" , filename);
    } else {
        println!("File path does not exist! Did you type in the file name correctly? : {}", filename);
    }
    Ok(())
}

fn create_file(path: &Path, file_name: &String) -> io::Result<()>{
    let full_path: PathBuf = path.join(file_name);
    File::create(&full_path)?;
    println!("New file {} has been created at {}", file_name, full_path.display());
    Ok(())
}

fn change_directory(dir: &str) -> io::Result<()>{
    let expanded_path = shellexpand::tilde(dir);
    let path = Path::new(&*expanded_path);

    match env::set_current_dir(&path) {
        Ok(_) => println!("Directory is now {}", path.display()),
        Err(e) => println!("Error: {}", e),
    }
    Ok(())
}