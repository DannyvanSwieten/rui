use rui::{
    app::{App, AppState, UIAppDelegate, WindowRequest},
    widget::button::TextButton,
    widget::container::Container,
};

struct Model;

impl AppState for Model {
    type MessageType = ();

    fn handle_message(&mut self, _: Self::MessageType) {}
}

fn main() {
    let app = pollster::block_on(App::new("Basic Example"));

    let delegate = UIAppDelegate::new().on_start(|app, _state| {
        app.ui_window_request(WindowRequest::new("Basic Example", 600, 400, |_state| {
            Box::new(Container::new(TextButton::new("Button", 18f32)))
        }));
    });

    app.run(delegate, Model);
}
