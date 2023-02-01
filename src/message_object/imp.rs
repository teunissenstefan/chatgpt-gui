use std::cell::RefCell;
use std::rc::Rc;

use glib::{ParamSpec, ParamSpecString, Value};
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use once_cell::sync::Lazy;

use super::MessageData;

#[derive(Default)]
pub struct MessageObject {
    pub data: Rc<RefCell<MessageData>>,
}

#[glib::object_subclass]
impl ObjectSubclass for MessageObject {
    const NAME: &'static str = "MessageObject";
    type Type = super::MessageObject;
}

impl ObjectImpl for MessageObject {
    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![
                ParamSpecString::builder("user").build(),
                ParamSpecString::builder("content").build(),
            ]
        });
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _id: usize, value: &Value, pspec: &ParamSpec) {
        match pspec.name() {
            "user" => {
                let input_value =
                    value.get().expect("The value needs to be of type `String`.");
                self.data.borrow_mut().user = input_value;
            }
            "content" => {
                let input_value = value
                    .get()
                    .expect("The value needs to be of type `String`.");
                self.data.borrow_mut().content = input_value;
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _id: usize, pspec: &ParamSpec) -> Value {
        match pspec.name() {
            "user" => self.data.borrow().user.to_value(),
            "content" => self.data.borrow().content.to_value(),
            _ => unimplemented!(),
        }
    }
}