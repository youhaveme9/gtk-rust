use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Button};

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

    // crteated a button with label and margin on all sides
    let button = Button::builder()
        .label("Press Me!")
        .margin_top(20)
        .margin_bottom(20)
        .margin_start(20)
        .margin_end(20)
        .build();

    // added an event listner to the button
    button.connect_clicked(|button| {
        button.set_label("Hello World");  // changed the label of the button when it is clicked
    });

    

    // creating a blank window and setting the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My First GTK App")
        .child(&button)
        .build();

    // present the window to the screen
    window.present();
}