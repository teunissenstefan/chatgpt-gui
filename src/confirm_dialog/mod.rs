mod imp;

use glib::{Object};
use gtk::{gio, glib, Application};

glib::wrapper! {
    pub struct ConfirmDialog(ObjectSubclass<imp::ConfirmDialog>)
        @extends gtk::ApplicationWindow, gtk::Dialog, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl ConfirmDialog {
    pub fn new(app: &Application) -> Self {
        Object::builder().property("application", app).build()
    }

    fn setup_callbacks(&self) {
        // self.imp()
        //     .entry
        //     .connect_activate(clone!(@weak self as confirm_dialog => move |_| {
        //         confirm_dialog.new_message(true);
        //     }));

        // self.imp().entry.connect_icon_release(
        //     clone!(@weak self as confirm_dialog => move |_,_| {
        //         confirm_dialog.new_message(true);
        //     }),
        // );
    }
}