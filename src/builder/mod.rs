use tokio::io::AsyncWriteExt;
use tokio::fs;
use tokio::task;
use std::env;

struct File {

    file_name: String,
    content: String,

}

pub enum ProjectType {

    Custom(String),
    Discord(String),

}

pub struct Builder {

    dir_name: String,
    files: Vec<File>,
    sub_dir: String

}

impl Builder {

    pub fn new(project_type: ProjectType) -> Self {

        let gitignore= File{file_name: ".gitignore".to_owned(), content: "# Ignore files".to_owned()};
        let requirements= File{file_name: "requirements.txt".to_owned(), content: "## Requirements".to_owned()};

        match project_type {

            ProjectType::Custom(dir_name) => {

                Builder {

                    dir_name,
                    files: vec![File{file_name: "README.md".to_owned(), content: "## README documentation".to_owned()},
                                gitignore,
                                requirements,
                                File{file_name: "src/main.py".to_owned(), content: "def main():\n\n\tprint('Hello World')\n\nif __name__ == '__main__':\n\n\tmain()".to_owned()}],
                    sub_dir: "src".to_owned()
                    
                }

            }

            ProjectType::Discord(dir_name) => {

                Builder {

                    dir_name,
                    files: vec![gitignore,
                                requirements,
                                File{file_name: "bot.py".to_owned(), content: "def main():\n\n\tprint('Hello World')\n\nif __name__ == '__main__':\n\n\tmain()".to_owned()}],
                    sub_dir: "cogs".to_owned()

                }

            }

        }

    }

    pub async fn create_custom_project(&self) -> std::io::Result<()> {

        fs::create_dir_all(&self.dir_name).await?;
        env::set_current_dir(&self.dir_name).unwrap();
        fs::create_dir_all(&self.sub_dir).await?;

        let mut tasks= vec![];

        for file in self.files.iter() {

            let file_name= file.file_name.clone();
            let file_contents= file.content.clone();

            let handle= task::spawn(async move {
                create_file(file_name, file_contents).await
            });

            tasks.push(handle);

        }

        for handle in tasks {

            handle.await??;

        }

        return Ok(())

    }

    
}

async fn create_file(file_name: String, file_contents: String) -> std::io::Result<()> {

    let mut buffer_file= fs::File::create(file_name).await?;

    buffer_file.write_all(file_contents.as_bytes()).await?;

    return Ok(())

}
