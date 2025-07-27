use super::code_op::{ AddCodeOp, EngageCodeOp };

#[derive(Clone, Debug)]
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
    fn change_name(&mut self, new_name: &String){
        self.name = new_name.clone();
    }
}

impl EngageCodeOp for SkeletonDataClump{
    fn add_op(&self, tokens: Vec::<String>, first_op: AddCodeOp, second_op: AddCodeOp) -> Self{

        match first_op{
            AddCodeOp::NAME => {},
            AddCodeOp::DATATYPE => {},
            AddCodeOp::REFERENCE => {},
            AddCodeOp::MODIFIER => {},
            AddCodeOp::RETURN => {},
            AddCodeOp::LIFETIME => {},
            AddCodeOp::NONE => {},
        }

        todo!();
    }
    fn create_op(tokens: Vec::<String>, first_op: AddCodeOp, second_op: AddCodeOp) -> Self{
        todo!();
    }
}
