mod spell_corrector;
use glib::clone;
use gtk::prelude::*;
use gtk::{gdk,gio,glib};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let application = gtk::Application::new(Some("com.gtk-rs.spelling_corrector"), Default::default());

    application.connect_activate(build_ui);
    application.run();
}

fn build_ui(app: &gtk::Application) {
    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .title("Spelling Corrector")
        .default_width(660)
        .default_height(420)
        .build();

    let display = gdk::Display::default().unwrap();
    
    let container = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .margin_top(24)
        .margin_bottom(24)
        .margin_start(24)
        .margin_end(24)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .spacing(24)
        .build();

    let title = gtk::Label::builder()
        .label("Spelling Corrector")
        .halign(gtk::Align::Start)
        .build();
    title.add_css_class("title-2");
    container.append(&title);

    let text_container = gtk::Box::builder()
        .halign(gtk::Align::Center)
        .orientation(gtk::Orientation::Horizontal)
        .spacing(24)
        .build();

    let text_to_be_corrected = gtk::Entry::builder()
        .placeholder_text("Type word to be corrected")
        .build();

    text_container.append(&text_to_be_corrected);

    let output = gtk::Entry::new();


    let go_btn = gtk::Button::with_label("Find Word");
    text_container.append(&go_btn);
    text_container.append(&output);

    let mut corrector = Rc::new(RefCell::new(spell_corrector::SpellCorrector::new()));


    corrector.borrow_mut().use_dictionary("notsobig.txt".to_string());
    go_btn.connect_clicked(clone!(@weak output,@weak text_to_be_corrected => move |_btn| {
        let word_to_use = text_to_be_corrected.text();
        
        let suggestion = corrector.borrow_mut().suggest_similar_word(word_to_use.as_str().to_string());

        match suggestion {
            Err(_) => output.set_text("No similar word found"),
            Ok(word) => output.set_text(&word)
        }
    }));

    container.append(&text_container);

    window.set_child(Some(&container));
    window.show();

}
