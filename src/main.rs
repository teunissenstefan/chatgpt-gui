mod message_object;
mod message_row;
mod window;

use gtk::prelude::*;
use gtk::{gio, Application};
use window::Window;

fn main() {
    gio::resources_register_include!("chatgpt.gresource")
        .expect("Failed to register resources.");

    let app = Application::builder()
        .application_id("org.gtk_rs.ChatGPT")
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &Application) {
    let window = Window::new(app);
    window.present();
}