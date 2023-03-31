use std::fmt::{Display, Formatter};

pub enum CommandType {
    Create,
    Load
}

pub struct Blog {
    pub name: &'static str,
}

impl Display for Blog{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name )
    }
}


pub struct BlogCommand {
    pub name: &'static str,
    pub action : CommandType,

}

impl BlogCommand {
    pub fn new_create(name: &'static str) -> BlogCommand {
        return BlogCommand { name, action: CommandType::Create };
    }

    pub fn new_load(name: &'static str) -> BlogCommand {
        return BlogCommand { name, action: CommandType::Load };
    }

}

