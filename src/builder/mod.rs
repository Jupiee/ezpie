use std::io::Write;
use std::fs;
use std::env;

struct File<'a> {

    file_name: &'a str,
    content: &'a str

}

pub struct Builder<'a> {

    dir_name: String,
    files: Vec<File<'a>>

}

impl<'a> Builder<'a> {

    pub fn new(dir_name: String) -> Self {

        Builder {

            dir_name,
            files: vec![File{file_name: "README.md", content: "## README documentation"},
                        File{file_name: ".gitignore", content: "# Ignore files"},
                        File{file_name: "requirements.txt", content: "## Requirements"},
                        File{file_name: "src/main.py", content: "def main():\n\n\tprint('Hello World')\n\nif __name__ == '__main__':\n\n\tmain()"}]

        }

    }

    pub fn create_project(&self) {

        fs::create_dir(&self.dir_name).expect("Unable to create directory");
        env::set_current_dir(&self.dir_name).unwrap();
        fs::create_dir("./src").expect("Unable to create directory");

        for file in self.files.iter() {

            self.create_file(file)

        }

    }

    fn create_file(&self, file: &File) {

        let mut buffer_file= fs::File::create(&file.file_name).unwrap();

        buffer_file.write_all(file.content.as_bytes()).unwrap();

    }

}