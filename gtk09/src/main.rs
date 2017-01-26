extern crate gdk;
extern crate gtk;

use gtk::CssProvider;
use gtk::prelude::*;
use gdk::Screen;
use gtk::StyleContext;

fn main() {
    gtk::init().expect("Failed to initialize GTK.");

    let css_str = include_str!("resources/style.css");
    let css_provider = CssProvider::new();
    css_provider.load_from_data(css_str).unwrap();
    StyleContext::add_provider_for_screen(
        &Screen::get_default().unwrap(), &css_provider, 1
    );

    let ui = include_str!("resources/mainWindow.glade");
    let builder = gtk::Builder::new_from_string(ui);
    
    let window1 : gtk::Window = builder.get_object("mainWindow").unwrap();
    window1.connect_delete_event(move |_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    let label: gtk::Label = builder.get_object("label1").unwrap();

    window1.show_all();
    gtk::main();
}
