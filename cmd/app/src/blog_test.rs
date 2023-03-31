use internal::blog_manager::{BlogManager, Manager};
use internal_domain::blog_types::{BlogCommand};

#[test]
fn load() {
    let blog_mg: Manager = BlogManager::new("test");
    let cmd = BlogCommand::new_create("test");
    let blog = blog_mg.create_blog(cmd);
    match blog {
        Some(v) => println!("{}", v),
        None => println!("{}", "error"),
    }
    let list = blog_mg.load_blog(BlogCommand::new_load("test"));
    match list {
        Some(mut v) => v.iter_mut().for_each(|x| println!("{}", x.name)),
        None => {}
    }
}
