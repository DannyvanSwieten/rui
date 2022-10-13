use crate::{canvas::Point, window::MouseEvent};

pub enum Event {
    MouseMove(MouseEvent),
    MouseEnter(MouseEvent),
    MouseLeave(MouseEvent),
    MouseUp(MouseEvent),
    MouseDown(MouseEvent),
    MouseDrag(MouseEvent),
}

impl Event {
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

    pub fn to_local(&self, position: &Point) -> Event {
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
