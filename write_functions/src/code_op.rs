

pub enum AddCodeOp{
    NAME,
    DATATYPE,
    REFERENCE,
    MODIFIER,
    RETURN,
    LIFETIME,
    NONE,
}

pub trait EngageCodeOp{
    fn add_op(&self, tokens: Vec::<String>, first_op: AddCodeOp, second_op: AddCodeOp) -> Self;
    fn create_op(tokens: Vec::<String>, op: AddCodeOp, second_op: AddCodeOp) -> Self;
}