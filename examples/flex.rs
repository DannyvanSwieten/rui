use rui::{
    app::{App, AppState, MessageCtx, UIAppDelegate, WindowRequest},
    widget::{button::TextButton, flex::Flex},
};

struct State;

impl AppState for State {
    type Message = ();

    fn handle_message(&mut self, _: Self::Message, _: &mut MessageCtx<Self>) {}
}

fn main() {
    let app = App::new();

    let request = WindowRequest::new("Flex", 600, 400, |_state| {
        Box::new(
            Flex::column()
                .push(TextButton::new("Button 1", 24.0))
                .push(TextButton::new("Button 2", 24.0))
                .push(TextButton::new("Button 3", 24.0))
                .push(
                    Flex::row()
                        .push(TextButton::new("Button 4", 24.0))
                        .push(TextButton::new("Button 5", 24.0))
                        .push(TextButton::new("Button 6", 24.0)),
                ),
        )
    });
    let delegate = UIAppDelegate::new(request);

    app.run(delegate, State);
}
