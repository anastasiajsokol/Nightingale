#[derive(Debug, Default)]
pub enum State {
    #[default]
    Inbox,
    Sent,
    Drafts,
    Compose,
}
