use alloc::{boxed::Box, string::String, vec::Vec};

use crate::println;

use super::command::Command;

pub struct EchoCommand {
    content: String,
}

impl Command for EchoCommand {
    fn parse(args: alloc::vec::Vec<&str>) -> alloc::boxed::Box<dyn Command>
    where
        Self: Sized,
    {
        Box::new(Self {
            content: String::from(args.iter().skip(1).map(|&a| a).collect::<Vec<_>>().join("")),
        })
    }

    fn execute(&self) {
        println!("{}", self.content);
    }

    fn name() -> String
    where
        Self: Sized,
    {
        String::from("echo")
    }
}
