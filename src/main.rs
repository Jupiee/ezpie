use std::io::Write;
use std::fs;
use std::env;
use cli_prompts::{ prompts::{Input, AbortReason}, DisplayPrompt};

fn dirname_validation(name: &str) -> Result<String, String> {

    // checks if the name has /, \ and : in it
    if name.len() > 0 && !name.contains("/") && !name.contains("\\") && !name.contains(":") {

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
fn make_file(dir_name: String, file_name: String, content: String) {
    
    let path: String;

    if check_dir(&dir_name) {

        path= format!("{}/{}.py", dir_name, file_name);

    } else {

        fs::create_dir("./".to_owned() + &dir_name).expect("Unable to create directory");

        path= format!("{}/{}.py", dir_name, file_name);

    }

    let mut pyfile= fs::File::create(path).unwrap();

    pyfile.write_all(content.as_bytes()).unwrap();

}

fn main() {

    let prompt: Result<String, AbortReason> = Input::new("Enter Project name", dirname_validation).display();

    match &prompt {

        Ok(name) => fs::create_dir(format!("./{}", name)).expect("Unable to create directory"),
        Err(reason) => println!("{:?}", reason)

    }

    let dir_name= prompt.unwrap();

    let main_content= "def main():\n\n    print('Hello World')\n\nif __name__ == '__main__':\n\n    main()".to_owned();

    env::set_current_dir(format!("./{}", dir_name)).unwrap();

    make_readme(&dir_name);

    make_file("src".to_owned(), "main".to_owned(), main_content);

    make_file("src".to_owned(), "__init__".to_owned(), "".to_owned());

    make_gitignore();

}