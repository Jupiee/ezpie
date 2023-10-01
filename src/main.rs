use std::io::Write;
use std::fs;
use std::env;
use cli_prompts::{ prompts::{Input, AbortReason}, DisplayPrompt};

fn dirname_validation(name: &str) -> Result<String, String> {

    // checks if the name has /, \ and : in it
    if check_dir(&name.to_owned()) {

        return Err("Directory already exists".into())

    }

    else if name.len() > 0 && !name.contains("/") && !name.contains("\\") && !name.contains(":") {

        return Ok(name.to_string().replace(" ", "-"))  

    }

    else {

        return Err("Invalid directory name".into())

    }

}

fn check_dir(dir_name: &String) -> bool {

    let metadata= fs::metadata(format!("{}", dir_name));

    match metadata {

        Ok(_) => return true,
        Err(_) => return false

    }

}

fn make_readme(dir_name: &String) {

    let content= format!("## README documentation for {}", dir_name);

    let mut file= fs::File::create("README.md").unwrap();

    file.write_all(content.as_bytes()).unwrap();

}

fn make_gitignore() {

    let content= format!(".DS_Store\n/src/__pycache__");

    let mut file= fs::File::create(".gitignore").unwrap();

    file.write_all(content.as_bytes()).unwrap()

}

// creates a directory with python file
fn make_src() {
    
    fs::create_dir("./src".to_owned()).expect("Unable to create directory");

    let main_content= "def main():\n\n\tprint('Hello World')\n\nif __name__ == '__main__':\n\n\tmain()";

    let mut main_file= fs::File::create("src/main.py").unwrap();

    fs::File::create("src/__init__.py").unwrap();

    main_file.write_all(main_content.as_bytes()).unwrap();

}

fn main() {

    let prompt: Result<String, AbortReason> = Input::new("Enter Project name", dirname_validation).display();

    match &prompt {

        Ok(name) => fs::create_dir(format!("./{}", name)).expect("Unable to create directory"),
        Err(reason) => println!("{:?}", reason)

    }

    let dir_name= prompt.unwrap();

    env::set_current_dir(format!("./{}", dir_name)).unwrap();

    make_readme(&dir_name);

    make_src();

    make_gitignore();

}