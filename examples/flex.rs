use rui::{
    app::{App, AppRequest, AppState, UIAppDelegate, WindowRequest},
    widget::{
        button::{ButtonStyle, TextButton},
        flex::Flex,
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
            "Flex",
            600,
            400,
            |_state| {
                Box::new(
                    Flex::column()
                        .push(TextButton::new("Button 1", 24.0).style(ButtonStyle::Outline))
                        .push(TextButton::new("Button 2", 24.0).style(ButtonStyle::Outline))
                        .push(TextButton::new("Button 3", 24.0).style(ButtonStyle::Outline))
                        .push(
                            Flex::row()
                                .push(TextButton::new("Button 4", 24.0).style(ButtonStyle::Outline))
                                .push(TextButton::new("Button 5", 24.0).style(ButtonStyle::Outline))
                                .push(
                                    TextButton::new("Button 6", 24.0).style(ButtonStyle::Outline),
                                ),
                        ),
                )
            },
        )));
    });

    app.run(delegate, State);
}
