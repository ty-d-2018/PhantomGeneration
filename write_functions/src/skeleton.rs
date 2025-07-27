use super::code_op::{ AddCodeOp, EngageCodeOp, Nested};


pub struct SkeletonDataClump{
    modifier: Option::<String>,
    name: String,
    parameters: Option::<Vec::<(String, String)>>,
    return_type: Option::<String>,
    code_block: Option::<Vec::<String>>,
}

impl SkeletonDataClump{
    pub fn new(name: &String) -> SkeletonDataClump{
        SkeletonDataClump{
            modifier: None,
            name: name.clone(),
            parameters: None,
            return_type: None,
            code_block: None,
        }
    }
}

impl EngageCodeOp for SkeletonDataClump{
    fn add_op(statement: Self, tokens: String, op: AddCodeOp, nested: Nested) -> Self{
        todo!();
    }
    fn create_op(tokens: String, op: AddCodeOp) -> Self{
        todo!();
    }
}
