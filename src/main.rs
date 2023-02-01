mod message_object;
mod message_row;
mod window;

use gio::SimpleAction;
use glib::clone;
use gtk::prelude::*;
use gtk::{gio, Application, MessageDialog, ResponseType};
use window::Window;

const APP_ID: &'static str = "org.gtk_rs.ChatGPT";

fn main() {
    gio::resources_register_include!("chatgpt.gresource")
        .expect("Failed to register resources.");

    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(build_ui);

    app.set_accels_for_action("win.close", &["<Ctrl>W"]);

    app.run();
}

fn build_ui(app: &Application) {
    let window = Window::new(app);

    let quit_dialog = MessageDialog::builder()
        .title("ChatGPT")
        .text("Are you sure you want to quit?")
        .secondary_text("Your chat history will be lost if you quit.")
        .modal(true)
        .build();
    quit_dialog.set_transient_for(Some(&window));
    quit_dialog.add_button("No", ResponseType::No);
    quit_dialog.add_button("Yes", ResponseType::Yes);

    let action_close = SimpleAction::new("close", None);
    action_close.connect_activate(clone!(@weak window => move |_, _| {
            let quit_dialog_instance = quit_dialog.clone();
            gtk::glib::MainContext::default().spawn_local(async move {
                let answer = quit_dialog_instance.run_future().await;
                if answer == ResponseType::Yes {
                    window.close();
                }
                quit_dialog_instance.hide();
            });
    }));
    window.add_action(&action_close);

    window.present();
}