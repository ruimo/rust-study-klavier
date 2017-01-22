extern crate gdk;
extern crate gtk;
extern crate cairo;
extern crate gdk_pixbuf;

use std::cell::RefCell;
use gtk::prelude::*;
use std::rc::Rc;
use gdk_pixbuf::Pixbuf;
use gdk::prelude::*;
use cairo::ImageSurface;
use cairo::Surface;
use cairo::Format;
use cairo::Context;
use cairo::prelude::*;

struct WindowState {
    clefsArea: gtk::DrawingArea,
    scoreArea: gtk::DrawingArea,
    velocityArea: gtk::DrawingArea,
    surface: ImageSurface
}

fn main() {
    gtk::init().expect("Failed to initialize GTK.");

    let ui = include_str!("resources/mainWindow.glade");
    let builder1 = gtk::Builder::new_from_string(ui);
    let window1 : gtk::Window = builder1.get_object("mainWindow").unwrap();

    window1.show_all();

    let builder2 = gtk::Builder::new_from_string(ui);
    let window2 : gtk::Window = builder2.get_object("mainWindow").unwrap();

    window2.show_all();
    gtk::main();
}
