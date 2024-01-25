mod builder;

use builder::Builder;
use builder::ProjectType;
use std::path::Path;
use std::time::Instant;
use std::process::exit;
use clap::{Command, Arg, ArgAction};
use tokio;

fn dirname_validation(name: &String) -> Result<String, String> {

    let path= Path::new(&name);

    if path.exists() {

        return Err("Directory already exists".to_owned())

    }

    if name.len() > 0 && !name.contains("/") && !name.contains("\\") && !name.contains(":") {

        return Ok(name.to_owned())  

    }

    else {

        return Err("Invalid directory name".to_owned())

    }

}

#[tokio::main]
async fn main() {

    let prompt= Command::new("Ezpie")
        .author("Jupie")
        .version("1.2.0")
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
    
    let now= Instant::now();

    match dirname_validation(dir_name) {

        Ok(name) => {
            
            let builder: Builder;

            if !discord {

                builder= Builder::new(ProjectType::Custom(name));

            }

            else {

                builder= Builder::new(ProjectType::Discord(name));

            }

            builder.create_custom_project().await.unwrap();

            println!("\nSuccessfully created in {:.2?}", now.elapsed());

        },

        Err(reason) => {

            println!("{:?}", reason);
            exit(1);

        }
    }

}