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
    fn change_modifier(&mut self, new_modifier: &String){
        self.modifier = Some(new_modifier.clone());
    }
    fn add_parameter(&mut self, name: &String, data_type: &String){
        let param_list: &mut Vec::<(String, String)> = match self.parameters.as_mut(){
            Some(v) => v,
            None => {
                self.parameters = Some(Vec::new());
                self.parameters.as_mut().unwrap()
            },
        };

        param_list.push((name.clone(), data_type.clone()));
    }

    fn change_return_type(&mut self, new_return_type: &String){
        self.return_type = Some(new_return_type.clone());
    }
}

impl EngageCodeOp for SkeletonDataClump{
    fn add_op(&self, tokens: Vec::<String>, first_op: AddCodeOp, second_op: AddCodeOp) -> Self{

        let mut new_skeleton_data: SkeletonDataClump = self.clone();
        
        let default_element: String = String::from("Undefined");
        let token_one: &String = tokens.get(0).unwrap_or(&default_element);
        let token_two: &String = tokens.get(1).unwrap_or(&default_element);

        match first_op{
            AddCodeOp::NAME => {
                if let AddCodeOp::DATATYPE = second_op{
                    new_skeleton_data.add_parameter(token_one, token_two);
                }else{
                    new_skeleton_data.change_name(token_one);
                }
            },
            AddCodeOp::DATATYPE => {},
            AddCodeOp::REFERENCE => {},
            AddCodeOp::MODIFIER => {
                new_skeleton_data.change_modifier(token_one);
            },
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
