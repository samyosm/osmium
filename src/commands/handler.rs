use alloc::{string::String, vec::Vec};
use hashbrown::HashMap;
use spin::Once;

use super::command::Command;

type CommandExecute = fn(Vec<&str>);
struct CommandRegistry {
    map: HashMap<String, CommandExecute>,
}

static COMMANDS: Once<CommandRegistry> = Once::new();

impl CommandRegistry {
    pub fn global() -> &'static CommandRegistry {
        COMMANDS.call_once(|| {
            let mut map = CommandRegistry::new();
            map.push::<super::time::TimeCommand>();
            map.push::<super::exit::ExitCommand>();
            map.push::<super::echo::EchoCommand>();

            map
        })
    }

    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn push<T: Command>(&mut self) {
        self.map.insert(T::name(), T::parse_and_execute);
    }

    pub fn get<T: Into<String>>(&self, name: T) -> Option<&CommandExecute> {
        self.map.get(&name.into())
    }
}

pub fn handle_input(input: &str) {
    let args: Vec<&str> = input.split_whitespace().collect();
    if let Some(&arg) = args.get(0) {
        if let Some(command) = CommandRegistry::global().get(arg) {
            command(args);
        }
    }
}
