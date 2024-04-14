slint::include_modules!();
use arboard::Clipboard;
use slint::SharedString;
mod filemanager;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;



    ui.on_generate_code({
        let ui_handle = ui.as_weak();
        move || {
            let line = filemanager::generate_line();
            let clone_line = &line.unwrap();

            let clip_line = String::from(clone_line);

            let string_line = SharedString::from(clone_line);
            
            let ui = ui_handle.unwrap();

            ui.set_code_property(string_line);
            
            // ======== Clipboarding code ========
            let mut clipboard = Clipboard::new().unwrap();

            let the_string = clip_line;
            clipboard.set_text(the_string).unwrap();
            // println!("But now the clipboard text should be: \"{}\"");
        }
    });

    let handle = ui.as_weak();
    
    let ui = handle.unwrap();

    let default_line = SharedString::from("Welcome to my generator");

    ui.set_code_property(default_line);
    // TODO - Write the line in another file called used_line.txt
    // TODO - remove the generated line from the main code.txt
    
    
    ui.run()
}
