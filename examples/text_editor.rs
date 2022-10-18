use rui::{
    app::{App, AppState, MessageCtx, UIAppDelegate, WindowRequest},
    widget::{center::Center, text_editor::TextBox},
};

struct State;

impl AppState for State {
    type Message = ();

    fn handle_message(&self, _: Self::Message, _: &mut MessageCtx<Self>) {}
}

fn main() {
    let app = App::new();

    let request = WindowRequest::new("TextBox Example", 400, 100, |_state| {
        Box::new(Center::new(TextBox::new("Text...")))
    });
    let delegate = UIAppDelegate::new(request);

    app.run(delegate, State);
}
