use std::iter::Iterator;

pub struct SK_Function{
    modifier: Modifier,
    pub name: String,
    parameters: Option::<Vec::(String, Box::<dyn PType>)>,
}

enum Modifier{
    Private,
    Public,
}

pub trait PType{
    fn get_name(&self) -> String;
}

impl SK_Function{
    pub fn get_p_iterator() -> ParamIterator{
        todo!();
    }
}

pub struct ParamIterator{
    index: usize,
    parameters: &Vec::<(String, Box::<dyn PType>)>,
}

impl ParamIterator{
    pub fn new(parameters: &Vec::<(String, Box::<dyn Ptype>)>) -> ParamIterator{
        ParamIterator{
            index: 0,
            parameters,
        }
    }
}

impl Iterator for ParamIterator{
    type Item = &(String, Box::<dyn Ptype>);

    fn next(&mut self) -> Option::<Self::Item>{
        previous_count = self.index;
        self.index += 1;

        self.parameters.get(previous_count)
    }
}

