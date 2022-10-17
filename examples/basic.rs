use rui::{
    app::{App, AppState, UIAppDelegate, WindowRequest},
    widget::button::ButtonStyle,
    widget::button::TextButton,
    widget::container::Container,
};

struct State;

impl AppState for State {
    type MessageType = ();

    fn handle_message(&mut self, _: Self::MessageType) {}
}

fn main() {
    let app = App::new();

    let delegate = UIAppDelegate::new().on_start(|app, _state| {
        app.request(rui::app::AppRequest::OpenWindowRequest(WindowRequest::new(
            "Basic Example",
            600,
            400,
            |_state| {
                Box::new(
                    Container::new(TextButton::new("Button", 24.0).style(ButtonStyle::Outline))
                        .with_padding(50.0),
                )
            },
        )));
    });

    app.run(delegate, State);
}
