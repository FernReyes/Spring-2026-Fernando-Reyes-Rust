
#[derive(Debug)]
enum FileOperation {
    List(String),               // Directory path
    Display(String),            // File path
    Create(String, String),     // File path and content
    Remove(String),             // File path
    Pwd,                        // Print working directory
}

use std::process::Command;
fn perform_operation(operation: FileOperation) {
    // Implement command execution based on the operation
}

//Function to make file when command asks for it.
fn create_file(file_path: &str, content: &str) {
    let user_command = format!("echo '{}' > {}", content, file_path);
    let output = Command::new("sh")
        .arg("-c")
        .arg(&user_command)
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        println!("File created successfully");
    } else {
        eprintln!("Failed to create file");
    }
}

use std::io;
fn main() {
    
    loop{
        
        //Gets user input.
        let mut input = String::new();
        let _ = io::stdin().read_line(&mut input);
        let opera_input: i32 = input.trim().parse().expect("Input is not vaild");

        //File operation menu with input number.

        if opera_input == 1{
            
        }
        else if opera_input == 2{
            
        }
        else if opera_input == 3{
            
        }
        else if opera_input == 4{
            
        }
        else if opera_input == 5{
            
        }
        else if opera_input == 0{
            break;
        }
        else{
            println!("Number is not valid.");
        }

    }
}
