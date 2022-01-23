#[derive(Default, Debug)]
pub struct WallState {
    pub wall_n: bool,
    pub wall_s: bool,
    pub wall_w: bool,
    pub wall_e: bool,
}

impl WallState {
    pub(super) fn top_right() -> Self {
        let mut wall_state = WallState::default();
        wall_state.wall_n = true;
        wall_state.wall_e = true;
        wall_state
    }

    pub(super) fn top_left() -> Self {
        let mut wall_state = WallState::default();
        wall_state.wall_n = true;
        wall_state.wall_w = true;
        wall_state
    }

    pub(super) fn bottom_left() -> Self {
        let mut wall_state = WallState::default();
        wall_state.wall_s = true;
        wall_state.wall_w = true;
        wall_state
    }

    pub(super) fn bottom_right() -> Self {
        let mut wall_state = WallState::default();
        wall_state.wall_s = true;
        wall_state.wall_e = true;
        wall_state
    }

    pub(super) fn top() -> Self {
        let mut wall_state = WallState::default();
        wall_state.wall_n = true;
        wall_state
    }

    pub(super) fn bottom() -> Self {
        let mut wall_state = WallState::default();
        wall_state.wall_s = true;
        wall_state
    }

    pub(super) fn right() -> Self {
        let mut wall_state = WallState::default();
        wall_state.wall_e = true;
        wall_state
    }

    pub(super) fn left() -> Self {
        let mut wall_state = WallState::default();
        wall_state.wall_w = true;
        wall_state
    }
}

pub struct Square {
    pub(super) wall_state: WallState
}

impl Square {
    pub fn new(wall_state: WallState) -> Self {
        Self {
            wall_state,
        }
    }
}