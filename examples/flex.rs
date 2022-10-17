use rui::{
    app::{App, AppState, UIAppDelegate, WindowRequest},
    widget::{
        button::{ButtonStyle, TextButton},
        flex::Column,
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
        app.ui_window_request(WindowRequest::new("Flex", 600, 400, |_state| {
            Box::new(
                Column::new()
                    .push(TextButton::new("Button 1", 24.0).style(ButtonStyle::Outline))
                    .push(TextButton::new("Button 2", 24.0).style(ButtonStyle::Outline))
                    .push(TextButton::new("Button 3", 24.0).style(ButtonStyle::Outline)),
            )
        }));
    });

    app.run(delegate, State);
}
