use std::cell::RefCell;

use glib::Binding;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate, Label};

#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/gtk_rs/ChatGPT/message_row.ui")]
pub struct MessageRow {
    #[template_child]
    pub user_label: TemplateChild<Label>,
    #[template_child]
    pub content_label: TemplateChild<Label>,
    pub bindings: RefCell<Vec<Binding>>,
}

#[glib::object_subclass]
impl ObjectSubclass for MessageRow {
    const NAME: &'static str = "MessageRow";
    type Type = super::MessageRow;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for MessageRow {}

impl WidgetImpl for MessageRow {}

impl BoxImpl for MessageRow {}