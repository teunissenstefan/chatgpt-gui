mod imp;

use glib::{clone, Object};
use gtk::{gio, glib, Application, NoSelection, SignalListItemFactory};
use gtk::{prelude::*, ListItem};
use gio::Settings;
use glib::subclass::prelude::ObjectSubclassIsExt;
use gtk::gio::SettingsBindFlags;
use glib::signal::Inhibit;
use crate::APP_ID;

glib::wrapper! {
    pub struct Preferences(ObjectSubclass<imp::Preferences>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

const SETTING_OPENAI_API_KEY: &'static str = "openai-api-key";

impl Preferences {
    pub fn new(app: &Application) -> Self {
        Object::builder().property("application", app).build()
    }

    fn setup_fields(&self) {
        let settings = Settings::new(APP_ID);

        let openai_api_key_entry = self.imp().openai_api_key_entry.get();

        settings
            .bind(SETTING_OPENAI_API_KEY, &openai_api_key_entry, "text")
            .flags(SettingsBindFlags::DEFAULT)
            .build();

        openai_api_key_entry.connect_changed(clone!(@weak settings => move |tree_selection| {
            settings
                .set_string(SETTING_OPENAI_API_KEY, &tree_selection.buffer().text())
                .expect("Could not set setting.");
        }));
    }

    fn setup_callbacks(&self) {
        self.imp()
            .close_button
            .connect_clicked(clone!(@weak self as window => move |_| {
                window.close();
            }));
    }
}