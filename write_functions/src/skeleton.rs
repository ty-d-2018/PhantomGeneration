use super::code_op::{ AddCodeOp, EngageCodeOp, Nested};


pub struct SkeletonDataClump{
    modifier: String,
    name: String,
    parameters: Vec::<(String, String)>,
    return_type: String,
}

