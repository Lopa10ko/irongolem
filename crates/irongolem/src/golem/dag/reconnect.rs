#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ReconnectType {
    None,
    #[default]
    Single,
    All,
}
