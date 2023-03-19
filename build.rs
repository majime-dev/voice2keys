extern crate winresource;

fn main() {
    let mut res = winresource::WindowsResource::new();

    res.set_icon("resources/v2k.ico")
        .set_manifest_file("resources/manifest.xml")
        .compile()
        .unwrap();
}
