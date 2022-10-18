pub trait AppState {
    type Message: Clone;

    fn handle_message(&self, msg: Self::Message);
}
