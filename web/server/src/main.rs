use annunaki_web::binary_splits::HelloTemplate;
use askama::Template;

fn main() {
    let hello = HelloTemplate { name: "world" };
    println!("{}", hello.render().unwrap());
}
