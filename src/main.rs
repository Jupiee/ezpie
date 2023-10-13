mod builder;

use builder::Builder;
use builder::ProjectType;
use std::path::Path;
use std::process::exit;
use clap::{Command, Arg, ArgAction};

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
        .arg(
            Arg::new("discord")
                .short('d')
                .long("discord")
                .action(ArgAction::SetTrue)
                .help("Creates project directory for discord.py")
        )
        .get_matches();

    let dir_name= prompt.get_one::<String>("Project name").unwrap();
    let discord= prompt.get_flag("discord");

    match dirname_validation(dir_name) {

        Ok(name) => {
            
            let builder: Builder;

            if !discord {

                builder= Builder::new(ProjectType::Custom(name));

            }

            else {

                builder= Builder::new(ProjectType::Discord(name));

            }

            builder.create_custom_project();

        },

        Err(reason) => {

            println!("{:?}", reason);
            exit(1);

        }
    }

}