

pub enum AddCodeOp{
    NAME,
    DATATYPE,
    REFERENCE,
    MODIFIER,
    RETURN,
    LIFETIME,
}

pub trait EngageCodeOp{
    fn add_op(statement: Self, tokens: String, op: AddCodeOp) -> Self;
    fn create_op(tokens: String, op: AddCodeOp) -> Self;
}