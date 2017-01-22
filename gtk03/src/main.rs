extern crate gtk;

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

struct WindowState {
    drawingArea: gtk::DrawingArea,
    color: RefCell<Color>
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

    let button: gtk::Button = builder.get_object("button1").unwrap();
    let drawingArea: gtk::DrawingArea = builder.get_object("drawingarea1").unwrap();
    drawingArea.set_size_request(300, 300);

    let windowState = Rc::new(
        WindowState {
            drawingArea: drawingArea,
            color: RefCell::new(
                Color { red: 0, green: 0, blue: 255 }
            )
        }
    );

    {
        let ws = windowState.clone();
        button.connect_clicked(move |_| {
            ws.color.borrow_mut().set_color(Color { red: 255, green: 0, blue: 0 });
            ws.drawingArea.queue_draw();
        });
    }

    {
        let ws = windowState.clone();
        windowState.drawingArea.connect_draw(move |widget, context| {
            println!("draw({}, {}) called.", widget.get_allocated_width(), widget.get_allocated_height());
            let color = ws.color.borrow();
            context.move_to(0f64, 0f64);
            context.set_line_width(10f64);
            context.set_source_rgb(f64::from(color.red) / 255.0, f64::from(color.green) / 255.0, f64::from(color.blue) / 255.0);
            context.line_to(300f64, 300f64);
            context.stroke();
            return Inhibit(false);
        });
    }

    window1.show_all();
    gtk::main();
}
