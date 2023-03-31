use internal_domain::blog_types::{Blog, BlogCommand, CommandType};

pub struct Manager {
    name: &'static str,
}

pub trait BlogManager {
    fn new(name: &'static str) -> Self;
    fn name(&self) -> &'static str;
    fn load_blog(&self, cmd: BlogCommand) -> Option<Vec<Blog>>;
    fn create_blog(&self, cmd: BlogCommand) -> Option<Blog>;
}

impl Manager {
    fn print(&self) {
        println!("I got called")
    }
}

impl BlogManager for Manager {
    fn new(name: &'static str) -> Self {
        Manager { name }
    }

    fn name(&self) -> &'static str {
        self.print();
        self.name
    }

    fn load_blog(&self, cmd: BlogCommand) -> Option<Vec<Blog>> {
        let action = cmd.action;
        match action {
            CommandType::Load =>  Some(vec![Blog { name: cmd.name }]),
            _ => None
        }
    }

    fn create_blog(&self, cmd: BlogCommand) -> Option<Blog> {
        let action = cmd.action;
        match action {
            CommandType::Create => Some(Blog { name: "created" }),
            _ => None
        }
    }
}

