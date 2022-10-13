use crate::{canvas::Point, window};

pub enum Event {
    Mouse(MouseEvent),
}

pub enum MouseEvent {
    MouseMove(window::MouseEvent),
    MouseEnter(window::MouseEvent),
    MouseLeave(window::MouseEvent),
    MouseUp(window::MouseEvent),
    MouseDown(window::MouseEvent),
    MouseDrag(window::MouseEvent),
}

impl MouseEvent {
    pub fn local_position(&self) -> &Point {
        match self {
            Self::MouseMove(event) => event.local_position(),
            Self::MouseEnter(event) => event.local_position(),
            Self::MouseLeave(event) => event.local_position(),
            Self::MouseUp(event) => event.local_position(),
            Self::MouseDown(event) => event.local_position(),
            Self::MouseDrag(event) => event.local_position(),
        }
    }

    pub fn to_local(&self, position: &Point) -> MouseEvent {
        match self {
            Self::MouseMove(event) => Self::MouseMove(event.to_local(position)),
            Self::MouseEnter(event) => Self::MouseEnter(event.to_local(position)),
            Self::MouseLeave(event) => Self::MouseLeave(event.to_local(position)),
            Self::MouseUp(event) => Self::MouseUp(event.to_local(position)),
            Self::MouseDown(event) => Self::MouseDown(event.to_local(position)),
            Self::MouseDrag(event) => Self::MouseDrag(event.to_local(position)),
        }
    }
}
