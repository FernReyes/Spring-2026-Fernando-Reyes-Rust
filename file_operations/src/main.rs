#[derive(Debug)]
#[derive(PartialEq)]
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
    match operation{
        FileOperation::List(directory_path) => {
            Command::new("ls").arg(directory_path).status().expect("Failed to execute ls");
        }

        FileOperation::Display(file_path) => {
            Command::new("cat").arg(file_path).status().expect("Failed to execute cat");
        }

        FileOperation::Create(file_path, content) => {
            let command = format!("echo '{}' > {}", content, file_path);
            Command::new("sh").arg("-c").arg(command).status().expect("Failed to create file");
            println!("File '{}' created successfully", file_path);
        }

        FileOperation::Remove(file_path) => {
            Command::new("rm").arg(&file_path).status().expect("Failed to remove file");
            println!("File '{}' removed successfully", file_path);
        }

        FileOperation::Pwd => {
            Command::new("pwd").status().expect("Failed to execute pwd");
        }
    }
}

use std::io;
fn main() {
    
    println!("Welcome to the File Operations Program!");
    loop{

        //Menu
        println!("File Operations Menu:");
        println!("1. List files in a directory");
        println!("2. Display file contents");
        println!("3. Create a new file");
        println!("4. Remove a file");
        println!("5. Print working directory");
        println!("0. Exit");
        println!("");
        println!("Enter your choice (0-5):");
        
        //Gets user input.
        let mut input = String::new();
        let _ = io::stdin().read_line(&mut input);
        let opera_input: i32 = input.trim().parse().expect("Input is not vaild");

        //Matching input to enum variable.
        if opera_input == 1{

            println!("Enter directory path:");
            let mut dir = String::new();
            let _ = io::stdin().read_line(&mut dir).unwrap();
            println!("");

            let op = FileOperation::List(dir.trim().to_string());
            perform_operation(op);
        }

        else if opera_input == 2{
            
            println!("Enter file path:");
            let mut file = String::new();
            let _ = io::stdin().read_line(&mut file).unwrap();
            println!(""); 

            let op = FileOperation::Display(file.trim().to_string());
            perform_operation(op);
        }

        else if opera_input == 3{

            println!("Enter file path:");
            let mut file = String::new();
            let _ = io::stdin().read_line(&mut file).unwrap();

            println!("Enter content:");
            let mut cont = String::new();
            let _ = io::stdin().read_line(&mut cont).unwrap();
            println!("");

            let op = FileOperation::Create(file.trim().to_string(), cont.trim().to_string());
            perform_operation(op);

        }

        else if opera_input == 4{
            
            println!("Enter file path:");
            let mut file = String::new();
            let _ = io::stdin().read_line(&mut file).unwrap();
            println!("");

            let op = FileOperation::Remove(file.trim().to_string());
            perform_operation(op);
        }

        else if opera_input == 5{
            println!("");
            let op = FileOperation::Pwd;
            perform_operation(op);
        }

        else if opera_input == 0{
            break;
        }

        else{
            println!("Number is not valid.");
        }
        println!("");
    }
    println!("Goodbye");
}