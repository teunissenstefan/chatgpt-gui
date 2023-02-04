mod imp;

use glib::{clone, Object};
use gtk::{gio, glib, Application};
use gtk::{prelude::*};
use gio::Settings;
use glib::subclass::prelude::ObjectSubclassIsExt;
use gtk::gio::SettingsBindFlags;
use crate::APP_ID;

glib::wrapper! {
    pub struct Preferences(ObjectSubclass<imp::Preferences>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

const SETTING_OPENAI_API_KEY: &'static str = "openai-api-key";
const SETTING_OPENAI_MODEL: &'static str = "openai-model";

impl Preferences {
    pub fn new(app: &Application) -> Self {
        Object::builder().property("application", app).build()
    }

    fn setup_fields(&self) {
        let settings = Settings::new(APP_ID);

        let openai_api_key_entry = self.imp().openai_api_key_entry.get();
        let openai_model_entry = self.imp().openai_model_entry.get();

        settings
            .bind(SETTING_OPENAI_API_KEY, &openai_api_key_entry, "text")
            .flags(SettingsBindFlags::DEFAULT)
            .build();
        settings
            .bind(SETTING_OPENAI_MODEL, &openai_model_entry, "text")
            .flags(SettingsBindFlags::DEFAULT)
            .build();

        openai_api_key_entry.connect_changed(clone!(@weak settings => move |entry| {
            settings
                .set_string(SETTING_OPENAI_API_KEY, &entry.buffer().text())
                .expect("Could not set setting.");
        }));
        openai_model_entry.connect_changed(clone!(@weak settings => move |entry| {
            settings
                .set_string(SETTING_OPENAI_MODEL, &entry.buffer().text())
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