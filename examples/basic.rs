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
    let app = pollster::block_on(App::new());

    let delegate = UIAppDelegate::new().on_start(|app, _state| {
        app.ui_window_request(WindowRequest::new("Basic Example", 600, 400, |_state| {
            Box::new(
                Container::new(TextButton::new("Button", 24f32).style(ButtonStyle::Outline))
                    .with_padding(50f32),
            )
        }));
    });

    app.run(delegate, State);
}
