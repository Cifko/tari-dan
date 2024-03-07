use std::{
    fs::File,
    path::PathBuf,
    process::{Child, Command, Stdio},
};

use crate::helpers::config::CONFIG;

pub struct Process {
    pub name: String,
    command: PathBuf,
    args: Vec<String>,
    child: Option<Child>,
}

impl Process {
    pub fn new(name: String, command: PathBuf, args: Vec<&str>) -> Process {
        Process {
            name,
            args: args.into_iter().map(String::from).collect(),
            command,
            child: None,
        }
    }

    pub fn run(&mut self) {
        let stdout = File::create(CONFIG.data_folder.join(format!("{}.log", self.name))).unwrap();
        let child = Command::new(&self.command)
            .args(&self.args)
            .stdout(Stdio::from(stdout))
            .spawn()
            .unwrap();
        self.child = Some(child);
    }
}

impl Drop for Process {
    fn drop(&mut self) {
        if let Some(mut child) = self.child.take() {
            child.kill().unwrap();
        }
    }
}
