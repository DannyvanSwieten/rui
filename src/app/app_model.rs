pub trait AppState {
    type MessageType;

    fn handle_message(&mut self, msg: Self::MessageType);
}
