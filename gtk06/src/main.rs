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

    let button: gtk::Button = builder.get_object("button1").unwrap();
    let clefsArea: gtk::DrawingArea = builder.get_object("clefsArea").unwrap();
    let clefsScrolledWindow: gtk::ScrolledWindow = builder.get_object("clefsScrolledWindow").unwrap();

    let scoreArea: gtk::DrawingArea = builder.get_object("scoreArea").unwrap();
    let scoreScrolledWindow: gtk::ScrolledWindow = builder.get_object("scoreScrolledWindow").unwrap();

    let velocityHeader: gtk::Frame = builder.get_object("velocityHeader").unwrap();
    let velocityArea: gtk::DrawingArea = builder.get_object("velocityArea").unwrap();
    let velocityScrolledWindow: gtk::ScrolledWindow = builder.get_object("velocityScrolledWindow").unwrap();

    let scoreVadjustment: gtk::Adjustment = scoreScrolledWindow.get_vadjustment().unwrap();
    let scoreHadjustment: gtk::Adjustment = scoreScrolledWindow.get_hadjustment().unwrap();
    let clefsVadjustment: gtk::Adjustment = clefsScrolledWindow.get_vadjustment().unwrap();
    let velocityHadjustment: gtk::Adjustment = velocityScrolledWindow.get_hadjustment().unwrap();
    scoreVadjustment.connect_value_changed(move |adj| {
        clefsVadjustment.set_value(adj.get_value());
    });
    scoreHadjustment.connect_value_changed(move |adj| {
        velocityHadjustment.set_value(adj.get_value());
    });

    clefsArea.set_size_request(clefpix.get_width(), clefpix.get_height());
    scoreArea.set_size_request(300, clefpix.get_height());
    velocityArea.set_size_request(300, 200);
    let windowState = Rc::new(
        WindowState {
            clefsArea: clefsArea,
            scoreArea: scoreArea,
            velocityArea: velocityArea,
            surface: isf
        }
    );

    {
        let ws = windowState.clone();
        windowState.clefsArea.connect_size_allocate(move |widget, alloc| {
            let w = alloc.width;
            let (curw, curh) = velocityHeader.get_size_request();
            velocityHeader.set_size_request(w, curh);
            clefsScrolledWindow.queue_resize_no_redraw();
        });
    }

    {
        let ws = windowState.clone();
        windowState.clefsArea.connect_draw(move |widget, context| {
            context.set_source_surface(&ws.surface, 0f64, 0f64);
            context.paint();
            return Inhibit(false);
        });
    }

    {
        let ws = windowState.clone();
        windowState.scoreArea.connect_draw(move |widget, context| {
            context.move_to(0f64, 0f64);
            context.set_line_width(10f64);
            context.line_to(300f64, 300f64);
            context.stroke();
            return Inhibit(false);
        });
    }

    {
        let ws = windowState.clone();
        windowState.velocityArea.connect_draw(move |widget, context| {
            context.move_to(0f64, 0f64);
            context.set_line_width(2f64);
            context.line_to(300f64, 300f64);
            context.stroke();
            return Inhibit(false);
        });
    }

    {
        let ws = windowState.clone();
        button.connect_clicked(move |_| {
            let (curw, curh) = ws.clefsArea.get_size_request();
            ws.clefsArea.set_size_request(curw - 100, curh);
        });
    }

    window1.show_all();
    gtk::main();
}
