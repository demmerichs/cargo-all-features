use std::{path, process};
use termcolor::WriteColor;

pub struct CargoTestRunner {
    command: process::Command,
    crate_name: String,
    feature_set: Vec<String>,
    working_dir: path::PathBuf,
}

impl CargoTestRunner {
    pub fn new(crate_name: String, feature_set: Vec<String>, working_dir: path::PathBuf) -> Self {
        let command = process::Command::new(&crate::cargo_cmd());

        let mut s = CargoTestRunner {
            crate_name,
            command,
            feature_set,
            working_dir,
        };
        s.arg("test");
        s.arg("--no-default-features");

        s
    }

    pub fn arg(&mut self, arg: &str) {
        self.command.arg(arg);
    }

    pub fn run(&mut self) {
        let mut stdout = termcolor::StandardStream::stdout(termcolor::ColorChoice::Auto);
        stdout
            .set_color(
                termcolor::ColorSpec::new()
                    .set_fg(Some(termcolor::Color::Cyan))
                    .set_bold(true),
            )
            .unwrap();
        print!("     Testing ");
        stdout.reset().unwrap();
        println!(
            "crate={} features=[{}]",
            self.crate_name,
            self.feature_set.join(", ")
        );

        let output = self
            .command
            .stdout(process::Stdio::inherit())
            .stderr(process::Stdio::inherit())
            .current_dir(&self.working_dir)
            .output()
            .unwrap(); // fixme

        if !output.status.success() {
            panic!("todo"); // fixme
        }
    }
}