

fn main() {
    slint_build::compile("gui/main.slint").expect("Slint build failed");
    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("assets/icon.ico");
        res.compile().unwrap();
    }
}
