use crate::{println, utils::time::get_mtl_datetime};
use alloc::{boxed::Box, string::String};

use super::command::Command;

pub struct TimeCommand {}

impl Command for TimeCommand {
    fn parse(_: alloc::vec::Vec<&str>) -> alloc::boxed::Box<dyn Command>
    where
        Self: Sized,
    {
        Box::new(Self {})
    }

    fn execute(&self) {
        let date_time = get_mtl_datetime();
        println!("{date_time}");
    }

    fn name() -> String
    where
        Self: Sized,
    {
        String::from("time")
    }
}
