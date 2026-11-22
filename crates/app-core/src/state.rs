use crate::Action;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Screen {
    #[default]
    Home,
    Activity,
    Settings,
}

impl Screen {
    pub const ALL: [Self; 3] = [Self::Home, Self::Activity, Self::Settings];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Home => "Home",
            Self::Activity => "Activity",
            Self::Settings => "Settings",
        }
    }

    pub const fn shortcut(self) -> char {
        match self {
            Self::Home => '1',
            Self::Activity => '2',
            Self::Settings => '3',
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct AppState {
    screen: Screen,
    count: u64,
}

impl AppState {
    pub const fn screen(&self) -> Screen {
        self.screen
    }

    pub const fn count(&self) -> u64 {
        self.count
    }
}

pub fn update(state: &mut AppState, action: Action) {
    match action {
        Action::Navigate(screen) => state.screen = screen,
        Action::Increment => state.count = state.count.saturating_add(1),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn navigation_keeps_the_counter() {
        let mut state = AppState::default();
        assert_eq!(state.screen(), Screen::Home);
        update(&mut state, Action::Increment);

        for screen in [Screen::Activity, Screen::Settings, Screen::Home] {
            update(&mut state, Action::Navigate(screen));
            assert_eq!(state.screen(), screen);
            assert_eq!(state.count(), 1);
        }
    }

    #[test]
    fn increment_keeps_the_current_screen() {
        let mut state = AppState::default();
        assert_eq!(state.count(), 0);
        update(&mut state, Action::Navigate(Screen::Activity));
        update(&mut state, Action::Increment);
        update(&mut state, Action::Increment);
        assert_eq!(state.screen(), Screen::Activity);
        assert_eq!(state.count(), 2);
    }

    #[test]
    fn counter_saturates_at_its_maximum() {
        let mut state = AppState {
            count: u64::MAX,
            ..AppState::default()
        };
        update(&mut state, Action::Increment);
        assert_eq!(state.count(), u64::MAX);
    }

    #[test]
    fn shortcut_mapping_matches_navigation() {
        for screen in Screen::ALL {
            assert_eq!(
                Action::from_shortcut(screen.shortcut()),
                Some(Action::Navigate(screen)),
            );
        }
        assert_eq!(Action::from_shortcut('I'), Some(Action::Increment));
        assert_eq!(Action::from_shortcut('i'), Some(Action::Increment));
        assert_eq!(Action::from_shortcut('x'), None);
    }
}
