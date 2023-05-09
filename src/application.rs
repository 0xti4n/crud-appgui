use crate::models::NewProperty;
use crate::{models::ScreenOutput, service::PropertyService};
use fltk::{app::*, browser::*, enums::*, input::*, prelude::*, window::*};
use fltk::{
    app::{self, App},
    button::Button,
    prelude::{GroupExt, WidgetExt},
    window::DoubleWindow,
};

const WIDGET_WIDTH: i32 = 70;
const WIDGET_HEIGHT: i32 = 25;
const WIDGET_PADDING: i32 = 10;

#[derive(Clone, Copy)]
enum Message {
    Create,
    Update,
    Delete,
    Select,
    Filter,
}

pub struct GUI {
    app: App,
    wind: DoubleWindow,
    sender: Sender<Message>,
    receiver: Receiver<Message>,
    service: PropertyService,
    filter_input: Input,
    list_browser: HoldBrowser,
    street_input: Input,
    number_input: Input,
    floor_input: Input,
    postal_code_input: Input,
    square_meters_input: Input,
    num_bathrooms_input: Input,
    num_bedrooms_input: Input,
    dwelling_type_input: Input,
    create_button: Button,
    update_button: Button,
    delete_button: Button,
}

impl GUI {
    pub fn new() -> GUI {
        let app = app::App::default().with_scheme(app::Scheme::Gtk);
        let wind = Window::default().with_label("CRUD");
        let (sender, receiver) = channel::<Message>();

        let filter_input = Input::default()
            .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
            .with_pos(WIDGET_PADDING + WIDGET_WIDTH * 2, WIDGET_PADDING)
            .with_label("Filter prefix:");

        let list_browser = HoldBrowser::default()
            .with_pos(
                WIDGET_PADDING,
                filter_input.y() + filter_input.height() + WIDGET_PADDING,
            )
            .with_size(WIDGET_WIDTH * 3, 300);

        let street_input = Input::default()
            .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
            .with_pos(
                list_browser.x() + list_browser.width() + WIDGET_PADDING + WIDGET_WIDTH * 2,
                list_browser.y(),
            )
            .with_label("Street:");

        let number_input = Input::default()
            .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
            .below_of(&street_input, WIDGET_PADDING)
            .with_label("Number:");

        let floor_input = Input::default()
            .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
            .below_of(&number_input, WIDGET_PADDING)
            .with_label("Floor:");

        let postal_code_input = Input::default()
            .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
            .below_of(&floor_input, WIDGET_PADDING)
            .with_label("Postal code:");

        let square_meters_input = Input::default()
            .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
            .below_of(&postal_code_input, WIDGET_PADDING)
            .with_label("Square meters:");

        let num_bathrooms_input = Input::default()
            .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
            .below_of(&square_meters_input, WIDGET_PADDING)
            .with_label("Num bathrooms:");

        let num_bedrooms_input = Input::default()
            .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
            .below_of(&num_bathrooms_input, WIDGET_PADDING)
            .with_label("Num bedrooms:");

        let dwelling_type_input = Input::default()
            .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
            .below_of(&num_bedrooms_input, WIDGET_PADDING)
            .with_label("Type of property:");

        let create_button = Button::default()
            .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
            .with_pos(WIDGET_PADDING, 350 + WIDGET_PADDING)
            .with_label("Create");

        let update_button = Button::default()
            .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
            .right_of(&create_button, WIDGET_PADDING)
            .with_label("Update");

        let delete_button = Button::default()
            .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
            .right_of(&update_button, WIDGET_PADDING)
            .with_label("Delete");

        let service = PropertyService::new();

        GUI {
            app: app,
            wind: wind,
            sender: sender,
            receiver: receiver,
            filter_input: filter_input,
            list_browser: list_browser,
            service,
            street_input,
            number_input,
            floor_input,
            postal_code_input,
            square_meters_input,
            num_bathrooms_input,
            num_bedrooms_input,
            dwelling_type_input,
            create_button: create_button,
            update_button: update_button,
            delete_button: delete_button,
        }
    }

    pub fn build(&mut self) {
        self.filter_input.set_trigger(CallbackTrigger::Changed);
        self.filter_input.emit(self.sender, Message::Filter);
        self.list_browser.emit(self.sender, Message::Select);
        self.create_button.emit(self.sender, Message::Create);
        self.update_button.emit(self.sender, Message::Update);
        self.update_button.deactivate();
        self.delete_button.emit(self.sender, Message::Delete);
        self.delete_button.deactivate();

        self.wind
            .set_size(500 + WIDGET_PADDING, 400 + WIDGET_PADDING);

        self.sender.send(Message::Filter);
    }

    fn clear_edit(&mut self) {
        self.street_input.set_value("");
        self.number_input.set_value("");
        self.floor_input.set_value("");
        self.postal_code_input.set_value("");
        self.square_meters_input.set_value("");
        self.num_bathrooms_input.set_value("");
        self.num_bedrooms_input.set_value("");
        self.dwelling_type_input.set_value("");
    }

    pub fn show(&mut self) {
        self.wind.end();
        self.wind.show();
        while self.app.wait() {
            match self.receiver.recv() {
                Some(Message::Create) => {
                    let new_property = NewProperty {
                        street: self.street_input.value(),
                        number: self.number_input.value(),
                        floor: self.floor_input.value(),
                        postal_code: self.postal_code_input.value(),
                        square_meters: self.square_meters_input.value().parse::<i32>().unwrap(),
                        num_bathrooms: self.num_bathrooms_input.value().parse::<i32>().unwrap(),
                        num_bedrooms: self.num_bedrooms_input.value().parse::<i32>().unwrap(),
                        dwelling_type: self.dwelling_type_input.value(),
                    };

                    let _ = self.service.create_post(new_property);
                    self.clear_edit();
                    self.sender.send(Message::Filter);
                }
                Some(Message::Update) => {
                    if self.list_browser.value() > 0 {
                        let text_selection =
                            self.list_browser.text(self.list_browser.value()).unwrap();
                        let data = self.service.get_properties().unwrap();
                        let search_result = data
                            .iter()
                            .filter(|e| e.clone().to_screen().eq_ignore_ascii_case(&text_selection))
                            .next();

                        match search_result {
                            Some(property) => {
                                let mut property = property.clone();
                                property.street = Some(self.street_input.value());
                                property.number = Some(self.number_input.value());
                                property.floor = Some(self.floor_input.value());
                                property.postal_code = Some(self.postal_code_input.value());
                                property.square_meters =
                                    Some(self.square_meters_input.value().parse::<i32>().unwrap());
                                property.num_bathrooms =
                                    Some(self.num_bathrooms_input.value().parse::<i32>().unwrap());
                                property.num_bedrooms =
                                    Some(self.num_bedrooms_input.value().parse::<i32>().unwrap());
                                property.dwelling_type = Some(self.dwelling_type_input.value());

                                let _ = self.service.update_property(property);
                                self.clear_edit();
                                self.sender.send(Message::Filter);
                                self.sender.send(Message::Select);
                            }
                            _ => {
                                println!("Not found.");
                            }
                        }
                    } else {
                        println!("Nothing to modify.");
                    }
                }
                Some(Message::Delete) => {
                    if self.list_browser.value() > 0 {
                        let text_selection =
                            self.list_browser.text(self.list_browser.value()).unwrap();
                        let data = self.service.get_properties().unwrap();
                        let search_result = data
                            .iter()
                            .filter(|e| e.clone().to_screen().eq_ignore_ascii_case(&text_selection))
                            .next();

                        match search_result {
                            Some(property) => {
                                let _ = self.service.delete_property(property.id.unwrap());
                                self.clear_edit();
                                self.sender.send(Message::Filter);
                                self.sender.send(Message::Select);
                            }
                            _ => {
                                println!("ELEMENTO NO ENCONTRADO!!!");
                            }
                        }
                    } else {
                        println!("NO HAY ELEMENTO PARA ELIMINAR!!!");
                    }
                }
                Some(Message::Select) => {
                    if self.list_browser.value() == 0 {
                        self.update_button.deactivate();
                        self.delete_button.deactivate();
                    } else {
                        let text_selection =
                            self.list_browser.text(self.list_browser.value()).unwrap();

                        let data = self.service.get_properties().unwrap();
                        let search_result = data
                            .iter()
                            .filter(|e| e.clone().to_screen().eq_ignore_ascii_case(&text_selection))
                            .next();

                        match search_result {
                            Some(property) => {
                                //self.ident_input .set_value(&property.id);
                                self.street_input
                                    .set_value(&property.street.clone().unwrap());
                                self.number_input
                                    .set_value(&property.number.clone().unwrap());
                                self.floor_input.set_value(&property.floor.clone().unwrap());
                                self.postal_code_input
                                    .set_value(&property.postal_code.clone().unwrap());
                                self.square_meters_input
                                    .set_value(&property.square_meters.unwrap().to_string());
                                self.num_bathrooms_input
                                    .set_value(&property.num_bathrooms.unwrap().to_string());
                                self.num_bedrooms_input
                                    .set_value(&property.num_bedrooms.unwrap().to_string());
                                self.dwelling_type_input.set_value(
                                    &property.dwelling_type.clone().unwrap().to_string(),
                                );
                                self.update_button.activate();
                                self.delete_button.activate();
                            }
                            _ => {
                                println!("Not found!!!");
                            }
                        }
                    }
                }
                Some(Message::Filter) => {
                    let prefix = self.filter_input.value().to_lowercase();
                    let filter_empty = prefix.trim().eq_ignore_ascii_case("");
                    self.list_browser.clear();

                    let Ok(data) = self.service.get_properties() else {
                        panic!("Not Properties!!!");
                    };
                    for (_, p) in data.iter().enumerate() {
                        if (p
                            .id
                            .unwrap()
                            .to_string()
                            .eq_ignore_ascii_case(prefix.as_str())
                            && !filter_empty)
                            || (filter_empty)
                        {
                            let item = p.to_screen();
                            self.list_browser.add(&item);
                        }
                        self.clear_edit();
                    }
                    self.clear_edit();
                    self.sender.send(Message::Select);
                }
                None => {}
                _ => {}
            }
        }
    }
}
