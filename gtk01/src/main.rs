extern crate gtk;

use gtk::prelude::*;

fn main() {
    gtk::init().expect("Failed to initialize GTK.");

    let ui = include_str!("resources/mainWindow.glade");
    let builder = gtk::Builder::new_from_string(ui);
    
    let window1 : gtk::Window = builder.get_object("mainWindow").unwrap();
    window1.connect_delete_event(move |_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    let button: gtk::Button = builder.get_object("button1").unwrap();
    let label: gtk::Label = builder.get_object("label1").unwrap();

    button.connect_clicked(move |_| {
        label.set_label("Clicked!");
    });

    window1.show_all();
    gtk::main();
}
