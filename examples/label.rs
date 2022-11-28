use rui::{
    app::{App, AppState, MessageCtx, UIAppDelegate, WindowRequest},
    widget::{expanded::Expanded, flex::Flex, label::Label, text_editor::TextBox},
};

struct State {
    label_text: String,
}

#[derive(Clone)]
enum Response {
    TextChanged,
}

impl AppState for State {
    type Message = String;
    type Response = Response;

    fn handle_message(
        &mut self,
        message: Self::Message,
        _: &mut MessageCtx<Self>,
    ) -> Self::Response {
        self.label_text = message;
        Response::TextChanged
    }
}

fn main() {
    let app = App::new();

    let request = WindowRequest::new("Label Example", 250, 100, |_state| {
        Box::new(
            Flex::column()
                .push(Expanded::new(
                    TextBox::new("Press enter to edit label")
                        .on_commit(|text, _, event_ctx| event_ctx.publish(text.to_string())),
                ))
                .push(Expanded::new(
                    Label::new("").with_provider(|state: &State| state.label_text.clone()),
                )),
        )
    });
    let delegate = UIAppDelegate::new(request);

    app.run(
        delegate,
        State {
            label_text: "No Text".to_string(),
        },
    );
}
