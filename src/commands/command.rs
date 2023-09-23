use alloc::{boxed::Box, string::String, vec::Vec};
use getargs::Options;

use crate::println;

pub trait Command {
    fn parse(args: Vec<&str>) -> Box<dyn Command>
    where
        Self: Sized;

    fn parse_and_execute(args: Vec<&str>)
    where
        Self: Sized,
    {
        Self::parse(args).execute();
    }

    fn execute(&self);

    fn name() -> String
    where
        Self: Sized;
}

pub struct LsCommand {
    dir: String,
    show_all: bool,
}

impl Command for LsCommand {
    // const NAME: &'static str = "";
    fn parse(args: Vec<&str>) -> Box<dyn Command> {
        let mut show_all = false;
        let mut opts = Options::new(args.iter().map(|&a| a));
        while let Ok(Some(opt)) = opts.next_opt() {
            match opt {
                getargs::Opt::Short('a') | getargs::Opt::Long("all") => show_all = true,
                _ => {}
            }
        }
        opts.next_positional();
        let dir = String::from(opts.next_positional().unwrap_or("."));
        Box::new(Self { dir, show_all })
    }

    fn name() -> String {
        String::from("ls")
    }

    fn execute(&self)
    where
        Self: Sized,
    {
        println!("LS: {:?}", self.dir);
    }
}

pub struct ExitCommand {}

impl Command for ExitCommand {
    fn parse(_args: Vec<&str>) -> Box<dyn Command> {
        Box::new(Self {})
    }

    fn name() -> String {
        String::from("exit")
    }
    fn execute(&self)
    where
        Self: Sized,
    {
        println!("Exit...");
    }
}
