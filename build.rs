extern crate winres;

// build.rs
fn main() {
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        // set icon for the EXE
        res.set_icon("./resources/game_icon.ico");
        res.compile().unwrap();
    }
}
