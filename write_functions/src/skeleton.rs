use super::code_op::{ AddCodeOp, EngageCodeOp, Nested};

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
    fn add_op(&self, tokens: String, op: AddCodeOp, nested: Nested) -> Self{
        match nested{
            _ => (),
        };

        match op{
            NAME => {},
            DATATYPE => {},
            REFERENCE => {},
            MODIFIER => {},
            RETURN => {},
            LIFETIME => {},
        }

        todo!();
    }
    fn create_op(tokens: String, op: AddCodeOp) -> Self{
        todo!();
    }
}
