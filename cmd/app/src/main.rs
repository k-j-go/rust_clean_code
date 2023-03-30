use std::ops::IndexMut;
use internal::blog_manager::{BlogManager, Manager};
use internal_domain::blog_types::{Blog, BlogCommand};

fn main() {
    let blog_mg: Manager = BlogManager::new("test");
    let cmd = BlogCommand::new_create("test");
    let blog = blog_mg.create_blog(cmd);
    println!("{}", blog.name);
    let mut list = blog_mg.load_blog( BlogCommand::new_load( "test"));
    list.iter_mut().for_each( |x| println!("{}", x.name))

}
