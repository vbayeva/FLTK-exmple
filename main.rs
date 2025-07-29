use fltk::{app, button::Button, dialog, prelude::*, window::Window, frame::Frame, button::CheckButton};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let app = app::App::default();
    
    let mut window = Window::new(100, 100, 400, 300, "Some cool stuff");
    
    let mut analyze_btn = Button::new(50, 50, 120, 30, "Describe file");

    let mut back_btn = Button::new(50, 20, 80, 30, "Back");
    back_btn.hide();
    
    let mut checkbox1 = CheckButton::new(50, 60, 150, 30, "1");
    checkbox1.hide();
    
    let mut checkbox2 = CheckButton::new(50, 90, 150, 30, "2");
    checkbox2.hide();
    
    let mut checkbox3 = CheckButton::new(50, 120, 150, 30, "3");
    checkbox3.hide();
    
    let mut file_btn = Button::new(50, 160, 100, 30, "Choose File");
    file_btn.hide();
    
    let mut file_info = Frame::new(160, 160, 200, 30, "No file selected");
    file_info.hide();
    
    let mut proceed_btn = Button::new(50, 200, 100, 30, "Let's do iiiit");
    proceed_btn.hide();

    let file_info_shared = Rc::new(RefCell::new(file_info.clone()));

    let mut extract_btn = Button::new(50, 100, 120, 30, "Extract");
    extract_btn.set_callback(|_| {
        println!("Extract!");
    });

    analyze_btn.set_callback({
        let mut back_btn = back_btn.clone();
        let mut checkbox1 = checkbox1.clone();
        let mut checkbox2 = checkbox2.clone();
        let mut checkbox3 = checkbox3.clone();
        let mut file_btn = file_btn.clone();
        let mut file_info = file_info.clone();
        let mut proceed_btn = proceed_btn.clone();
        let mut first_btn = analyze_btn.clone();
        let mut second_btn = extract_btn.clone();

        move |_| {
            first_btn.hide();
            second_btn.hide();
            
            back_btn.show();
            checkbox1.show();
            checkbox2.show();
            checkbox3.show();
            file_btn.show();
            file_info.show();
            proceed_btn.show();
            
            first_btn.parent().unwrap().redraw();
        }
    });

    back_btn.set_callback({
        let mut back_btn = back_btn.clone();
        let mut checkbox1 = checkbox1.clone();
        let mut checkbox2 = checkbox2.clone();
        let mut checkbox3 = checkbox3.clone();
        let mut file_btn = file_btn.clone();
        let mut file_info = file_info.clone();
        let mut proceed_btn = proceed_btn.clone();
        let mut first_btn = analyze_btn.clone();
        let mut second_btn = extract_btn.clone();
        
        move |_| {
            back_btn.hide();
            checkbox1.hide();
            checkbox2.hide();
            checkbox3.hide();
            file_btn.hide();
            file_info.hide();
            proceed_btn.hide();
            
            first_btn.show();
            second_btn.show();
            
            first_btn.parent().unwrap().redraw();
        }
    });


    file_btn.set_callback({
        let file_info_shared = file_info_shared.clone();
        
        move |_| {
            let mut dialog = dialog::NativeFileChooser::new(dialog::NativeFileChooserType::BrowseFile);
            dialog.set_title("Select a file");
            dialog.set_filter("*.txt");
            dialog.show();
            
            let filename = dialog.filename();
            if !filename.as_os_str().is_empty() {
                let file_name = filename.file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string();

                file_info_shared.borrow_mut().set_label(&file_name);

                println!("Selected file: {}", filename.to_string_lossy());
            }
        }
    });
    
    // check: the file is selected, some checkboxes are enabled, only then the button is not greyed
    proceed_btn.set_callback(|_| {
        println!("doiing the thing");
    });
    
    window.end();
    window.show();
    
    app.run().unwrap();
}
