use rui::application::{Application, ApplicationModel, UIApplicationDelegate};

struct Model;

impl ApplicationModel for Model {
    type MessageType = ();

    fn handle_message(&mut self, _: Self::MessageType) {}
}

fn main() {
    let application = Application::new("Basic Example");
    let delegate = UIApplicationDelegate::new();

    application.run(delegate, Model);
}
