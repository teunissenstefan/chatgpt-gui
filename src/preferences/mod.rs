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
const SETTING_OPENAI_MAX_TOKENS: &'static str = "openai-max-tokens";

impl Preferences {
    pub fn new(app: &Application) -> Self {
        Object::builder().property("application", app).build()
    }

    fn setup_fields(&self) {
        let settings = Settings::new(APP_ID);

        let openai_max_tokens = settings.int("openai-max-tokens");

        let openai_api_key_entry = self.imp().openai_api_key_entry.get();
        let openai_model_entry = self.imp().openai_model_entry.get();
        let openai_max_tokens_entry = self.imp().openai_max_tokens_entry.get();

        settings
            .bind(SETTING_OPENAI_API_KEY, &openai_api_key_entry, "text")
            .flags(SettingsBindFlags::DEFAULT)
            .build();
        openai_api_key_entry.connect_changed(clone!(@weak settings => move |entry| {
            settings
                .set_string(SETTING_OPENAI_API_KEY, &entry.buffer().text())
                .expect("Could not set setting.");
        }));

        settings
            .bind(SETTING_OPENAI_MODEL, &openai_model_entry, "text")
            .flags(SettingsBindFlags::DEFAULT)
            .build();
        openai_model_entry.connect_changed(clone!(@weak settings => move |entry| {
            settings
                .set_string(SETTING_OPENAI_MODEL, &entry.buffer().text())
                .expect("Could not set setting.");
        }));

        openai_max_tokens_entry.set_text(&openai_max_tokens.to_string());
        //Not actually connecting?
        // openai_max_tokens_entry.connect_insert_text(move |entry, new_text, position| {
        //     println!("help: {}", "xddd");
        //     // Check if new_text contains only digits
        //     if new_text.chars().all(|c| c.is_digit(10)) {
        //         // Allow text to be inserted
        //     } else {
        //         // Prevent text from being inserted
        //         glib::signal::signal_stop_emission_by_name(entry, "insert_text");
        //     }
        // });

        openai_max_tokens_entry.connect_changed(clone!(@weak settings => move |entry| {
            let text = entry.buffer().text();
            if !text.is_empty() {
                let mut int_to_save: i32 = text.parse::<i32>().unwrap();
                if int_to_save < 1 {
                    int_to_save = 1;
                }
                settings
                    .set_int(SETTING_OPENAI_MAX_TOKENS, int_to_save)
                    .expect("Could not set setting.");
            }
        }));
    }

    fn setup_callbacks(&self) {
        self.imp()
            .close_button
            .connect_clicked(clone!(@weak self as window => move |_| {
                window.close();
            }));

        let openai_max_tokens_entry = self.imp().openai_max_tokens_entry.get();
        self.imp()
            .openai_max_tokens_down_button
            .connect_clicked(clone!(@weak openai_max_tokens_entry as entry => move |_| {
                let old_int: i32 = entry.buffer().text().parse().unwrap();
                let mut int_to_save: i32 = old_int.checked_sub(1).unwrap_or(-2_147_483_648);
                if int_to_save < 1 {
                    int_to_save = 1;
                }
                entry.set_text(&int_to_save.to_string());
            }));
        self.imp()
            .openai_max_tokens_up_button
            .connect_clicked(clone!(@weak openai_max_tokens_entry as entry => move |_| {
                let old_int: i32 = entry.buffer().text().parse().unwrap();
                let int_to_save: i32 = old_int.checked_add(1).unwrap_or(2_147_483_647);
                entry.set_text(&int_to_save.to_string());
            }));
    }
}