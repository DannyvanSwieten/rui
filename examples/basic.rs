use rui::{
    application::{Application, ApplicationModel, UIApplicationDelegate, WindowRequest},
    widget::button::TextButton,
    widget::container::Container,
};

struct Model;

impl ApplicationModel for Model {
    type MessageType = ();

    fn handle_message(&mut self, _: Self::MessageType) {}
}

fn main() {
    let application = pollster::block_on(Application::new("Basic Example"));
    let delegate = UIApplicationDelegate::new().on_start(|app, _state| {
        app.ui_window_request(WindowRequest::new("Basic Example", 600, 400, |_state| {
            Box::new(Container::new(TextButton::new("Button", 18f32)))
        }));
    });

    application.run(delegate, Model);
}
