use alloc::{boxed::Box, string::String, vec::Vec};

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
