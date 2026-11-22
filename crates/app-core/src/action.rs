use crate::Screen;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    Navigate(Screen),
    Increment,
}

impl Action {
    pub fn from_shortcut(key: char) -> Option<Self> {
        match key {
            'i' | 'I' => Some(Self::Increment),
            _ => Screen::ALL
                .into_iter()
                .find(|screen| screen.shortcut() == key)
                .map(Self::Navigate),
        }
    }
}
