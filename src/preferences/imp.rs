use glib::subclass::InitializingObject;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate, Button, Entry};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/teunissenstefan/ChatGPT/preferences.ui")]
pub struct Preferences {
    #[template_child]
    pub openai_api_key_entry: TemplateChild<Entry>,
    #[template_child]
    pub openai_model_entry: TemplateChild<Entry>,
    #[template_child]
    pub close_button: TemplateChild<Button>,
}

#[glib::object_subclass]
impl ObjectSubclass for Preferences {
    const NAME: &'static str = "Preferences";
    type Type = super::Preferences;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(class: &mut Self::Class) {
        class.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for Preferences {
    fn constructed(&self) {
        self.parent_constructed();

        let obj = self.obj();
        obj.setup_fields();
        obj.setup_callbacks();
    }
}

impl WidgetImpl for Preferences {}

impl WindowImpl for Preferences {}

impl ApplicationWindowImpl for Preferences {}