use rui::{
    app::{App, AppState, UIAppDelegate, WindowRequest},
    widget::button::ButtonStyle,
    widget::container::Container,
    widget::{button::TextButton, Widget},
};

struct State;

impl AppState for State {
    type MessageType = ();

    fn handle_message(&mut self, _: Self::MessageType) {}
}

fn build_first_window() -> Box<dyn Widget<State>> {
    Box::new(
        Container::new(
            TextButton::new("Open new window", 24.0)
                .style(ButtonStyle::Outline)
                .on_click(|app, _state| {
                    app.ui_window_request(WindowRequest::new("Second Window", 600, 400, |_| {
                        build_second_window()
                    }));
                }),
        )
        .with_padding(50f32),
    )
}
fn build_second_window() -> Box<dyn Widget<State>> {
    Box::new(
        Container::new(
            TextButton::new("You opened a second window", 24.0).style(ButtonStyle::Outline),
        )
        .with_padding(50f32),
    )
}

fn main() {
    let app = pollster::block_on(App::new());

    let delegate = UIAppDelegate::new().on_start(|app, _| {
        app.ui_window_request(WindowRequest::new("Window 1", 600, 400, |_| {
            build_first_window()
        }));
    });

    app.run(delegate, State);
}
