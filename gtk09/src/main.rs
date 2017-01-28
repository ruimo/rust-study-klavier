extern crate gdk_sys;
extern crate gtk;
extern crate gdk;

use gdk::EventMask;
use gdk::EventType;
use gdk::EventButton;
use gdk::Event;
use std::cell::RefCell;
use gtk::prelude::*;
use std::rc::Rc;

struct Color {
    red: u8,
    green: u8,
    blue: u8
}

impl Color {
    fn set_color(&mut self, new_color: Color) {
        *self = new_color;
    }
}

fn main() {
    gtk::init().expect("Failed to initialize GTK.");

    let ui = include_str!("resources/mainWindow.glade");
    let builder = gtk::Builder::new_from_string(ui);
    let window1 : gtk::Window = builder.get_object("mainWindow").unwrap();

    window1.connect_delete_event(move |_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    let drawingArea: gtk::DrawingArea = builder.get_object("drawingarea1").unwrap();
    drawingArea.set_size_request(300, 300);
    drawingArea.add_events(gdk_sys::GDK_BUTTON_PRESS_MASK.bits() as i32);
    drawingArea.add_events(gdk_sys::GDK_BUTTON_RELEASE_MASK.bits() as i32);

    {
        drawingArea.connect_event(|_, e| {
            let clone = e.clone();
            match e.get_event_type() {
                EventType::ButtonPress => {
                    let res: Result<EventButton, Event> = clone.downcast();
                    println!("pressed: {:?}", res.unwrap().get_position());
                },
                EventType::ButtonRelease => {
                    let res: Result<EventButton, Event> = clone.downcast();
                    println!("released: {:?}", res.unwrap().get_position());
                },
                _ => {}
            }
            return Inhibit(false);
        });
    }

    drawingArea.connect_draw(move |widget, context| {
        context.move_to(0f64, 0f64);
        context.set_line_width(10f64);
        context.set_source_rgb(0.0, 0.0, 0.0);
        context.line_to(300f64, 300f64);
        context.stroke();
        return Inhibit(false);
    });

    window1.show_all();
    gtk::main();
}
