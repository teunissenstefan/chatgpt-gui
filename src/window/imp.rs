use std::cell::RefCell;

use glib::subclass::InitializingObject;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib, CompositeTemplate, Entry, ListView};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/gtk_rs/ChatGPT/window.ui")]
pub struct Window {
    #[template_child]
    pub entry: TemplateChild<Entry>,
    #[template_child]
    pub messages_list: TemplateChild<ListView>,
    pub messages: RefCell<Option<gio::ListStore>>,
}

#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "ChatGPT";
    type Type = super::Window;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(class: &mut Self::Class) {
        class.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for Window {
    fn constructed(&self) {
        self.parent_constructed();

        let obj = self.obj();
        obj.setup_messages();
        obj.setup_callbacks();
        obj.setup_factory();
    }
}

impl WidgetImpl for Window {}

impl WindowImpl for Window {}

impl ApplicationWindowImpl for Window {}