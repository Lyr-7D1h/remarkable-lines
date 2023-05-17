use super::crdt::CrdtId;

#[derive(Debug)]
pub struct LwwValue<T> {
    pub timestamp: CrdtId,
    pub value: T,
}
