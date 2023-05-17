use super::crdt::CrdtId;

#[derive(Debug, Clone)]
pub struct LwwValue<T> {
    pub timestamp: CrdtId,
    pub value: T,
}
