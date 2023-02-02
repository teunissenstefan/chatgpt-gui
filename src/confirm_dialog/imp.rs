use glib::subclass::InitializingObject;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate, Button};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/gtk_rs/ChatGPT/confirm_dialog.ui")]
pub struct ConfirmDialog {
    #[template_child]
    pub no_button: TemplateChild<Button>,
    #[template_child]
    pub yes_button: TemplateChild<Button>,
}

#[glib::object_subclass]
impl ObjectSubclass for ConfirmDialog {
    const NAME: &'static str = "ConfirmDialog";
    type Type = super::ConfirmDialog;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(class: &mut Self::Class) {
        class.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for ConfirmDialog {
    fn constructed(&self) {
        self.parent_constructed();

        let obj = self.obj();
        obj.setup_callbacks();
    }
}

impl WidgetImpl for ConfirmDialog {}

impl WindowImpl for ConfirmDialog {}

impl ApplicationWindowImpl for ConfirmDialog {}