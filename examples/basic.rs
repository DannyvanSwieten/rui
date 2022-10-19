use rui::{
    app::{App, AppState, MessageCtx, UIAppDelegate, WindowRequest},
    widget::{button::TextButton, container::Container},
};

struct State;

impl AppState for State {
    type Message = ();

    fn handle_message(&mut self, _: Self::Message, _: &mut MessageCtx<Self>) {}
}

fn main() {
    let app = App::new();

    let request = WindowRequest::new("Basic Example", 600, 400, |_state| {
        Box::new(Container::new(TextButton::new("Button", 24.0)).with_padding(50.0))
    });
    let delegate = UIAppDelegate::new(request);

    app.run(delegate, State);
}
