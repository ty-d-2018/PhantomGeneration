
pub enum Op{
    Create,
    Add,
    Take,
    Set,
}

pub fn create() -> Option::<Op>{
    Some(Op::Create)
}