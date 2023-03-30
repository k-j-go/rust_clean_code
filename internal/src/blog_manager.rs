use internal_domain::blog_types::{Blog, BlogCommand};

pub struct Manager {
    name: &'static str,
}

pub trait BlogManager {
    fn new(name: &'static str) -> Self;
    fn name(&self) -> &'static str;
    fn load_blog(&self, cmd: BlogCommand) -> Vec<Blog>;
    fn create_blog(&self, cmd: BlogCommand) -> Blog;
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

    fn load_blog(&self, cmd: BlogCommand) -> Vec<Blog> {
        return vec![Blog { name: cmd.name }];
    }

    fn create_blog(&self, cmd: BlogCommand) -> Blog {
        return Blog { name: "created" }
    }
}

