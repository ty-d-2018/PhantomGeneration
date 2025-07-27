

pub enum AddCodeOp{
    NAME,
    DATATYPE,
    REFERENCE,
    MODIFIER,
    RETURN,
    LIFETIME,
}

pub enum Nested{
    Empty,
    Parameter,
    Block,
}

pub trait EngageCodeOp{
    fn add_op(&self, tokens: String, op: AddCodeOp, nested: Nested) -> Self;
    fn create_op(tokens: String, op: AddCodeOp) -> Self;
}