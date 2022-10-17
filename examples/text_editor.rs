use rui::{
    app::{App, AppState, UIAppDelegate, WindowRequest},
    widget::container::Container,
    widget::text_editor::TextBox,
};

struct State;

impl AppState for State {
    type MessageType = ();

    fn handle_message(&mut self, _: Self::MessageType) {}
}

fn main() {
    let app = App::new();

    let delegate = UIAppDelegate::new().on_start(|app, _state| {
        app.ui_window_request(WindowRequest::new("TextBox Example", 400, 100, |_state| {
            Box::new(Container::new(TextBox::new("Text...")).with_padding(50.0))
        }));
    });

    app.run(delegate, State);
}
