pub enum CommandType {
    Create,
    Load
}

pub struct Blog {
    pub name: &'static str,
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

