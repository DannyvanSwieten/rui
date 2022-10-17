use rui::{
    app::{App, AppState, UIAppDelegate, WindowRequest},
    widget::{center::Center, text_editor::TextBox},
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
            Box::new(Center::new(TextBox::new("Text...")))
        }));
    });

    app.run(delegate, State);
}
