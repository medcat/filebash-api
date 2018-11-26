#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Modify<V> {
    Set(V),
    Keep,
    Default,
}
