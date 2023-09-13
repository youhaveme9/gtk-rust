use gtk::prelude::*;
use gtk::{self, glib, Application, ApplicationWindow, Button};
use std::cell::Cell;
use std::rc::Rc;
use glib::clone;

const APP_ID: &str = "org.gtk_rs.HelloWorld1";

fn main() -> glib::ExitCode {

    // creates a nuw application with application ID
    let app = Application::builder().application_id(APP_ID).build();

    // connect to "activate" signal of 'app'
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {

    let number = Rc::new(Cell::new(0));

    // crteated a button with label and margin on all sides
    let button = Button::builder()
        .label("Press Me!")
        .margin_top(20)
        .margin_bottom(20)
        .margin_start(20)
        .margin_end(20)
        .build();

    let button_inc = Button::builder()
        .label("Increase")
        .margin_top(20)
        .margin_bottom(20)
        .margin_start(20)
        .margin_end(20)
        .build();

    let button_dec = Button::builder()
        .label("Decrease")
        .margin_top(20)
        .margin_bottom(20)
        .margin_start(20)
        .margin_end(20)
        .build();

    // added an event listner to the button
    button.connect_clicked(|button| {
        button.set_label("Hello World");  // changed the label of the button when it is clicked
    });

    // button_inc.connect_clicked(clone!(@strong number => move |_| {
    //     number.set(number.get() + 1);
    // }));
    // button_inc.connect_clicked(clone!(@strong number => move |_| {
    //     number.set(number.get() +1);
    //     println!("{:?}", number);
    // }));
    
    button_dec.connect_clicked(move |_| {number.set(number.get() +1);
        println!("{:?}", number);
    });
    

    let gtk_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();
    gtk_box.append(&button);
    gtk_box.append(&button_inc);
    gtk_box.append(&button_dec);

    

    // creating a blank window and setting the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My First GTK App")
        .child(&gtk_box)
        .build();

    // present the window to the screen
    window.present();
}