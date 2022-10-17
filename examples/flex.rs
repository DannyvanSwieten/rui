use rui::{
    app::{App, AppState, UIAppDelegate, WindowRequest},
    widget::{
        button::{ButtonStyle, TextButton},
        flex::Flex,
    },
};

struct State;

impl AppState for State {
    type MessageType = ();

    fn handle_message(&self, _: Self::MessageType) {}
}

fn main() {
    let app = App::new();

    let delegate = UIAppDelegate::new().on_start(|app| {
        app.request(rui::app::AppRequest::OpenWindowRequest(WindowRequest::new(
            "Flex",
            600,
            400,
            |_state| {
                Box::new(
                    Flex::column()
                        .push(TextButton::new("Button 1", 24.0))
                        .push(TextButton::new("Button 2", 24.0))
                        .push(TextButton::new("Button 3", 24.0))
                        .push(
                            Flex::row()
                                .push(TextButton::new("Button 4", 24.0))
                                .push(TextButton::new("Button 5", 24.0))
                                .push(TextButton::new("Button 6", 24.0)),
                        ),
                )
            },
        )));
    });

    app.run(delegate, State);
}
