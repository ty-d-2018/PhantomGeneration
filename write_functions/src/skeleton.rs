use std::iter::Iterator;

pub struct SK_Function{
    modifier: Modifier,
    pub name: String,
    parameters: Option::<Vec::<(String, Box::<dyn PType>)>>,
}

enum Modifier{
    Private,
    Public,
}

pub trait PType{
    fn get_name(&self) -> String;
}

impl<'a> SK_Function{
    pub fn get_p_iterator() -> ParamIterator<'a>{
        todo!();
    }
}

pub struct ParamIterator<'a>{
    index: usize,
    parameters: &'a Vec::<(String, Box::<dyn PType>)>,
}

impl<'a> ParamIterator<'a>{
    pub fn new(parameters: &'a Vec::<(String, Box::<dyn PType>)>) -> Self{
        ParamIterator{
            index: 0,
            parameters,
        }
    }
}

impl<'a> Iterator for ParamIterator<'a>{
    type Item = &'a (String, Box::<dyn PType>);

    fn next(&mut self) -> Option::<Self::Item>{
        let mut previous_count = self.index;
        self.index += 1;

        self.parameters.get(previous_count)
    }
}

