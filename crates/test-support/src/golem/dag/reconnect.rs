#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ReconnectType {
    #[default]
    None,
    Single,
    All,
}
