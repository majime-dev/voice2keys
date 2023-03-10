extern crate winresource;

fn main() {
    let mut res = winresource::WindowsResource::new();
    res.set_icon("resources/v2k.ico").compile().unwrap();
}
