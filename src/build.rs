extern crate winres;

fn main() {
  if cfg!(target_os = "windows") {
    let mut res = winres::WindowsResource::new();
    res.set_icon("src/epik.ico"); // yes, we have an icon
    res.compile().unwrap();
  }
}
