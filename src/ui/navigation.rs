use bevy::prelude::*;

#[derive(Component)]
pub struct Focusable {
    pub order: usize,
    pub group: FocusGroup,
}

impl Focusable {
    pub fn new(order: usize, group: FocusGroup) -> Self {
        Self { order, group }
    }

    pub fn global(order: usize) -> Self {
        Self::new(order, FocusGroup::Global)
    }

    pub fn shop(order: usize) -> Self {
        Self::new(order, FocusGroup::Shop)
    }
}

#[derive(Component, Default)]
pub struct Focused;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FocusGroup {
    Global,
    Shop,
}

impl Default for FocusGroup {
    fn default() -> Self {
        FocusGroup::Global
    }
}

#[derive(Clone, Copy, Debug)]
pub enum FocusDirection {
    Next,
    Previous,
}

#[derive(Resource, Default)]
pub struct FocusState {
    pub active_group: FocusGroup,
    pub focused_entity: Option<Entity>,
    pub pending_entity: Option<Entity>,
    pub pending_direction: Option<FocusDirection>,
    pub dirty: bool,
}

impl FocusState {
    pub fn set_group(&mut self, group: FocusGroup) {
        if self.active_group != group {
            self.active_group = group;
            self.focused_entity = None;
            self.pending_entity = None;
            self.pending_direction = Some(FocusDirection::Next);
            self.dirty = true;
        }
    }

    pub fn request_focus(&mut self, entity: Entity) {
        self.pending_entity = Some(entity);
    }

    pub fn request_move(&mut self, direction: FocusDirection) {
        self.pending_direction = Some(direction);
    }

    pub fn clear(&mut self) {
        self.focused_entity = None;
        self.pending_entity = None;
        self.pending_direction = None;
        self.dirty = true;
    }
}
