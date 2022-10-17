use rui::{
    app::{App, AppRequest, AppState, UIAppDelegate, WindowRequest},
    widget::{
        button::{ButtonStyle, TextButton},
        center::Center,
        container::Container,
        flex::Flex,
        Widget,
    },
};

struct State;

impl AppState for State {
    type MessageType = ();

    fn handle_message(&mut self, _: Self::MessageType) {}
}

fn main() {
    let app = App::new();

    let delegate = UIAppDelegate::new().on_start(|app, _state| {
        app.request(AppRequest::OpenWindow(WindowRequest::new(
            "Solfege",
            600,
            400,
            |_state| {
                Box::new(Center::new(
                    Flex::row()
                        .with_spacing(5.0)
                        .push(button("Do"))
                        .push(button("Re"))
                        .push(button("Mi"))
                        .push(button("Fa"))
                        .push(button("So"))
                        .push(button("La"))
                        .push(button("Ti")),
                ))
            },
        )));
    });

    app.run(delegate, State);
}

fn button(name: &str) -> impl Widget<State> {
    Container::new(TextButton::new(name, 24.0).style(ButtonStyle::Outline))
}
