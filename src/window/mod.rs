mod imp;

use glib::{clone, Object, MainContext, PRIORITY_DEFAULT};
use gtk::subclass::prelude::*;
use gtk::{gio, glib, Application, NoSelection, SignalListItemFactory};
use gtk::{prelude::*, ListItem};
use std::io::*;
use gtk::gio::Settings;
use crate::message_object::MessageObject;
use crate::message_row::MessageRow;
use crate::{APP_ID, window};
use curl::easy::{Easy, List};
use serde::{Serialize, Deserialize};
use serde_json::json;
use std::thread;

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

#[derive(Serialize, Deserialize, Debug)]
struct ResultChoice {
    text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    finish_reason: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ResultMain {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    choices: Option<Vec<ResultChoice>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<ApiError>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ApiError {
    message: String,
    #[serde(rename = "type")]
    error_type: String,
}

fn trim_newline(mut s: String) -> String {
    if s.starts_with('\n') {
        s.remove(0);
    }
    if s.starts_with('\n') {
        s.remove(0);
    }
    return s;
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
                window.send_message();
            }));

        self.imp().entry.connect_icon_release(
            clone!(@weak self as window => move |_,_| {
                window.send_message();
            }),
        );
    }

    fn convert_result_to_object(returned: &String) -> Result<ResultMain> {
        let json_result: ResultMain = serde_json::from_str(returned)?;
        Ok(json_result)
    }

    fn send_request(msg: &String) -> std::result::Result<String, String> {
        let mut returned = Vec::new();

        let settings = Settings::new(APP_ID);
        let openai_api_key = settings.string("openai-api-key").to_string();
        let openai_model = settings.string("openai-model").to_string();
        let openai_max_tokens = settings.int("openai-max-tokens");

        let body_prompt = msg;
        let json_data = json!({
            "model": openai_model,
            "prompt": body_prompt,
            "max_tokens": openai_max_tokens,
            "n": 1,
            "temperature": 0,
            "user": format!("{:x}", md5::compute(whoami::username()))
        }).to_string();
        let mut data = json_data.as_bytes();

        let mut easy = Easy::new();
        easy.post(true);
        easy.url("https://api.openai.com/v1/completions").unwrap();
        easy.post_field_size(data.len() as u64).unwrap();

        //Headers
        let mut list = List::new();
        let mut header_bearer_token = "Authorization: Bearer ".to_string();
        header_bearer_token.push_str(&openai_api_key);
        list.append(&header_bearer_token).unwrap();
        list.append("Content-Type: application/json").unwrap();
        easy.http_headers(list).unwrap();

        //Send JSON body
        let mut transfer = easy.transfer();
        transfer.read_function(|buf| {
            Ok(data.read(buf).unwrap_or(0))
        }).unwrap();

        //Handle response
        transfer.write_function(|new_data| {
            returned.extend_from_slice(new_data);
            Ok(new_data.len())
        }).unwrap();
        transfer.perform().unwrap();
        drop(transfer);

        let s = match std::str::from_utf8(&*returned) {
            Ok(v) => v,
            Err(e) => return Err(format!("Invalid UTF-8 sequence: {}", e)),
        };

        let r: ResultMain = match window::Window::convert_result_to_object(&s.to_string()) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Failed to deserialize API response: {}", e);
                eprintln!("Raw response was: {}", s);
                return Err(format!("Failed to deserialize OpenAI API response: {}", e));
            }
        };

        // Check for API errors
        if let Some(error) = r.error {
            return Err(format!("OpenAI API Error: {}", error.message));
        }

        //TODO: handle "length" finish_reason

        let choices = r.choices.ok_or("No choices in API response".to_string())?;
        let first_choice: &ResultChoice = choices.first().ok_or("Empty choices array".to_string())?;
        let response_text = trim_newline(first_choice.text.to_string());
        return Ok(response_text);
    }

    fn add_message(&self, user: bool, msg: &String) {
        let from_who;
        if user {
            from_who = "You";
        } else {
            from_who = "ChatGPT";
        }
        let message = MessageObject::new(from_who.parse().unwrap(), msg.to_string());
        self.messages().append(&message);
    }

    fn send_message(&self) {
        let buffer = self.imp().entry.buffer();
        let content = buffer.text();
        if content.is_empty() {
            return;
        }
        buffer.set_text("");
        self.add_message(true, &content);
        let obj = self;

        let (sender, receiver) = MainContext::channel(PRIORITY_DEFAULT);
        let sender = sender.clone();
        thread::spawn(move || {
            sender.send("OPENAI_CHATGPT_BUTTON_DISABLE".to_string()).expect("Could not send through channel");
            match window::Window::send_request(&content) {
                Ok(response) => {
                    sender.send(response).expect("Could not send through channel");
                }
                Err(error_msg) => {
                    sender.send(format!("Error: {}", error_msg)).expect("Could not send through channel");
                }
            }
        });
        receiver.attach(
            None,
            clone!(@weak obj => @default-return Continue(false),
                    move |message| {
                        if message == "OPENAI_CHATGPT_BUTTON_DISABLE" {
                            obj.imp().entry.set_sensitive(false);
                        } else {
                            obj.add_message(false, &message);
                            obj.imp().entry.set_sensitive(true);
                            obj.imp().entry.get().grab_focus();
                        }
                        Continue(true)
                    }
            ),
        );
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