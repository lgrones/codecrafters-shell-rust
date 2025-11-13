use crate::commands::Command;

pub struct Echo {
    args: Vec<String>,
}

impl Echo {
    pub fn new(args: &[&str]) -> Self {
        Echo {
            args: args.into_iter().map(|x| x.to_string()).collect(),
        }
    }
}

impl Command for Echo {
    fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("{}", self.args.join(" "));
        Ok(())
    }
}
