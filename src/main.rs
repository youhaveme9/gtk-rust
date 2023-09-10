use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow};

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

    // creating a blank window and setting the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My First GTK App")
        .build();

    // present the window to the screen
    window.present();
}