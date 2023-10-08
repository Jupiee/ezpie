use std::io::Write;
use std::path::Path;
use std::fs;
use std::env;
use std::process::exit;
use clap::{Command, Arg};

struct File<'a> {

    file_name: &'a str,
    content: &'a str

}

fn dirname_validation(name: &String) -> Result<String, String> {

    if check_dir(&name.to_owned()) {

        return Err("Directory already exists".to_owned())

    }

    else if name.len() > 0 && !name.contains("/") && !name.contains("\\") && !name.contains(":") {

        return Ok(name.to_owned())  

    }

    else {

        return Err("Invalid directory name".to_owned())

    }

}

fn check_dir(dir_name: &String) -> bool {
    
    let path= Path::new(&dir_name);

    return path.exists();

}

fn create_file(file: &File) {
    
    let mut buffer_file= fs::File::create(&file.file_name).unwrap();

    buffer_file.write_all(file.content.as_bytes()).unwrap();

}

// creates a directory with python file
fn create_project(dir_name: String) {

    let files= vec![File{file_name: "README.md", content: "## README documentation"},
                            File{file_name: ".gitignore", content: "# Ignore files"},
                            File{file_name: "requirements.txt", content: "## Requirements"},
                            File{file_name: "src/main.py", content: "def main():\n\n\tprint('Hello World')\n\nif __name__ == '__main__':\n\n\tmain()"}];

    fs::create_dir(&dir_name).expect("Unable to create directory");
    env::set_current_dir(dir_name).unwrap();
    fs::create_dir("./src").expect("Unable to create directory");

    for file in files.iter() {

        create_file(file)

    }

}

fn main() {

    let prompt= Command::new("Ezpie")
        .author("Jupie")
        .version("1.0.0")
        .about("Build Python projects blazingly fast")
        .arg(
            Arg::new("Project name")
                .required(true)
                .help("Provide the name of the project without spaces")
        )
        .get_matches();

    let dir_name= prompt.get_one::<String>("Project name").unwrap();
    
    match dirname_validation(dir_name) {
        Ok(name) => create_project(name),
        Err(reason) => {

            println!("{:?}", reason);
            exit(1);

        }
    }

}