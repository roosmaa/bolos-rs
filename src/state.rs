
pub trait Store {
    type Action: Copy;

    fn process_action(&mut self, _action: Self::Action) {}
}

#[derive(Copy, Clone)]
pub enum BasicAction {
    Previous,
    Next,
    Confirm,
}
