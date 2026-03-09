use std::io;
use std::process::Command;

enum FileOperation {
    List(String),
    Display(String),
    Create(String, String),
    Remove(String),
    Pwd,
}

fn input() -> String { // function to avoid repeating input code many times
    let mut words = String::new(); //variable to store user input
    io::stdin().read_line(&mut words).unwrap(); //read input from terminal
    words.trim().to_string() //remove newline and return clean string
}

fn perform_operation(operation: FileOperation) {//Function to run commands based on enum
    match operation {

        FileOperation::List(path) => {
            Command::new("ls").arg(path).status().expect("Failed to execute ls");
        }

        FileOperation::Display(path) => {
            Command::new("cat").arg(path).status().expect("Failed to execute cat");
        }

        FileOperation::Create(path, content) => {
            let command_user = format!("echo '{}' > {}", content, path); // Build shell command string

            let result = Command::new("sh").arg("-c").arg(&command_user).output().expect("Failed to execute command");

            if result.status.success() { //check if command executed successfully
                println!("File '{}' created successfully.", path); // success message
            } else {
                println!("Failed to create file."); //error message
            }
        }

        FileOperation::Remove(path) => {
            let result = Command::new("rm").arg(&path).output().expect("Failed to execute rm");

            if result.status.success() { // Ccheck success
                println!("File '{}' removed successfully.", path); //success message
            } else {
                println!("Failed to remove file."); //error message
            }
        }

        FileOperation::Pwd => {
            println!("Current working directory:"); // label before running pwd
            Command::new("pwd").status().expect("Failed to execute pwd");
        }
    }
}

fn main() {
    println!("Welcome to the File Operations Program!"); // welcome message

    loop { // Loop of the menu
        println!("\nFile Operations Menu:"); //formatting line
        println!("1. List files in a directory");
        println!("2. Display file contents");
        println!("3. Create a new file");
        println!("4. Remove a file");
        println!("5. Print working directory");
        println!("0. Exit");
        println!("Enter your choice (0-5):");

        let choice = input(); //menu selection

            match choice.as_str() {

                "1" => {
                    println!("Enter directory path:");
                    perform_operation(FileOperation::List(input()));
                }

                "2" => {
                    println!("Enter file path:");
                    perform_operation(FileOperation::Display(input()));
                }

                "3" => {
                    println!("Enter file path:");
                    let path = input();

                    println!("Enter content:");
                    let content = input();

                    perform_operation(FileOperation::Create(path, content));
                }

                "4" => {
                    println!("Enter file path:");
                    perform_operation(FileOperation::Remove(input()));
                }

                "5" => perform_operation(FileOperation::Pwd),

                "0" => {
                    println!("Goodbye!");
                    break;
                }

                _ => println!("Invalid menu option."),
            }
        }
    }