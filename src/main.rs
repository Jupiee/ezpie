use std::io::Write;
use std::path::Path;
use std::fs;
use std::env;
use cli_prompts::{ prompts::{Input, AbortReason}, DisplayPrompt};

struct File {

    file_name: String,
    content: String

}

fn dirname_validation(name: &str) -> Result<String, String> {

    // checks if the name has /, \ and : in it
    if check_dir(&name.to_owned()) {

        return Err("Directory already exists".to_owned())

    }

    else if name.len() > 0 && !name.contains("/") && !name.contains("\\") && !name.contains(":") {

        return Ok(name.to_owned().replace(" ", "-"))  

    }

    else {

        return Err("Invalid directory name".to_owned())

    }

}

fn check_dir(dir_name: &String) -> bool {
    
    let path= Path::new(&dir_name);

    return path.exists();

}

fn make_file(file: File) {
    
    let mut buffer_file= fs::File::create(file.file_name).unwrap();

    buffer_file.write_all(file.content.as_bytes()).unwrap();

}

// creates a directory with python file
fn make_src() {
    
    fs::create_dir("./src").expect("Unable to create directory");

    let main_content= "def main():\n\n\tprint('Hello World')\n\nif __name__ == '__main__':\n\n\tmain()";

    let mut main_file= fs::File::create("src/main.py").unwrap();

    fs::File::create("src/__init__.py").unwrap();

    main_file.write_all(main_content.as_bytes()).unwrap();

}

fn main() {

    let files= [File{file_name: "README.md".to_owned(), content: "## README documentation".to_owned()},
                            File{file_name: ".gitignore".to_owned(), content: "# Ignore files".to_owned()},
                            File{file_name: "requirements.txt".to_owned(), content: "## Requirements".to_owned()}];

    let prompt: Result<String, AbortReason> = Input::new("Enter Project name", dirname_validation).display();

    match &prompt {

        Ok(name) => fs::create_dir( name).expect("Unable to create directory"),
        Err(reason) => println!("{:?}", reason)

    }

    let dir_name= prompt.unwrap();

    env::set_current_dir(dir_name).unwrap();

    for file in files {

        make_file(file);

    }

    make_src();

}