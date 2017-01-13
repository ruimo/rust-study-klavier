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
    drawingArea: gtk::DrawingArea,
    surface: ImageSurface
}

fn main() {
    gtk::init().expect("Failed to initialize GTK.");

    let clefpix = match Pixbuf::new_from_file("images/clefsImage.gif") {
        Err(e) => {
            println!("Err: {}.", e);
            return;
        }
        Ok(p) => p
    };
    let sharppix = match Pixbuf::new_from_file("images/sharp.gif") {
        Err(e) => {
            println!("Err: {}.", e);
            return;
        }
        Ok(p) => p
    };

    let isf = ImageSurface::create(Format::Rgb24, clefpix.get_width(), clefpix.get_height());
    let ctx = Context::new(&isf);
    ctx.set_source_pixbuf(&clefpix, 0f64, 0f64);
    ctx.paint();
    ctx.set_source_pixbuf(&sharppix, 100f64, 100f64);
    ctx.paint();
    isf.flush();

    let ui = include_str!("resources/mainWindow.glade");
    let builder = gtk::Builder::new_from_string(ui);
    let window1 : gtk::Window = builder.get_object("mainWindow").unwrap();

    window1.connect_delete_event(move |_, _| {
        gtk::main_quit();
        Inhibit(false)
    });


    let drawingArea: gtk::DrawingArea = builder.get_object("drawingarea1").unwrap();
    drawingArea.set_size_request(clefpix.get_width(), clefpix.get_height());
    let windowState = Rc::new(
        WindowState {
            drawingArea: drawingArea,
            surface: isf
        }
    );

    {
        let ws = windowState.clone();
        windowState.drawingArea.connect_draw(move |widget, context| {
            context.set_source_surface(&ws.surface, 0f64, 0f64);
            context.paint();
            return Inhibit(false);
        });
    }

    window1.show_all();
    gtk::main();
}
