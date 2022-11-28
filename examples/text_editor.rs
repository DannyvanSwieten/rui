use rui::{
    app::{App, AppState, MessageCtx, UIAppDelegate, WindowRequest},
    widget::{center::Center, container::Container, text_editor::TextBox},
};

struct State;

impl AppState for State {
    type Message = ();
    type Response = ();

    fn handle_message(&mut self, _: Self::Message, _: &mut MessageCtx<Self>) {}
}

fn main() {
    let app = App::new();

    let request = WindowRequest::new("TextBox Example", 400, 100, |_state| {
        Box::new(Container::new(Center::new(TextBox::new("Text..."))).with_padding(15.0))
    });
    let delegate = UIAppDelegate::new(request);

    app.run(delegate, State);
}
