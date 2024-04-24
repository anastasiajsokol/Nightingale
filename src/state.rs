#[derive(Debug, Default, PartialEq)]
pub enum State {
    #[default]
    Inbox,
    Sent,
    Drafts,
    Compose,
}

#[derive(Debug, Default)]
pub struct InboxState {
    pub scroll: usize,
}