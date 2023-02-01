mod imp;

use glib::{clone, Object};
use gtk::subclass::prelude::*;
use gtk::{gio, glib, Application, NoSelection, SignalListItemFactory};
use gtk::{prelude::*, ListItem};

use crate::message_object::MessageObject;
use crate::message_row::MessageRow;

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(app: &Application) -> Self {
        Object::builder().property("application", app).build()
    }

    fn messages(&self) -> gio::ListStore {
        self.imp()
            .messages
            .borrow()
            .clone()
            .expect("Could not get current messages.")
    }

    fn setup_messages(&self) {
        let model = gio::ListStore::new(MessageObject::static_type());

        self.imp().messages.replace(Some(model));

        let selection_model = NoSelection::new(Some(&self.messages()));
        self.imp().messages_list.set_model(Some(&selection_model));
    }

    fn setup_callbacks(&self) {
        self.imp()
            .entry
            .connect_activate(clone!(@weak self as window => move |_| {
                window.new_message(true);
            }));

        self.imp().entry.connect_icon_release(
            clone!(@weak self as window => move |_,_| {
                window.new_message(true);
            }),
        );
    }

    fn new_message(&self, user: bool) {
        let buffer = self.imp().entry.buffer();
        let content = buffer.text();
        if content.is_empty() {
            return;
        }
        buffer.set_text("");

        let from_who;
        if user {
            from_who = "You    ";
        }else{
            from_who = "ChatGPT";
        }
        let message = MessageObject::new(from_who.parse().unwrap(), content);
        self.messages().append(&message);
    }

    fn setup_factory(&self) {
        let factory = SignalListItemFactory::new();

        factory.connect_setup(move |_, list_item| {
            let message_row = MessageRow::new();
            list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .set_child(Some(&message_row));
        });

        factory.connect_bind(move |_, list_item| {
            let message_object = list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .item()
                .and_downcast::<MessageObject>()
                .expect("The item has to be an `MessageObject`.");

            let message_row = list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .child()
                .and_downcast::<MessageRow>()
                .expect("The child has to be a `MessageRow`.");

            message_row.bind(&message_object);
        });

        factory.connect_unbind(move |_, list_item| {
            let message_row = list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .child()
                .and_downcast::<MessageRow>()
                .expect("The child has to be a `MessageRow`.");

            message_row.unbind();
        });

        self.imp().messages_list.set_factory(Some(&factory));
    }
}