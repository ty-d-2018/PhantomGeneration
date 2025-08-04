use std::collections::HashMap;
use std::iter::Iterator;

pub fn create_label(name: &String) -> String{
    name.clone()
}

pub fn element_one(){
    todo!();
}

pub struct Element{
    data: &String
}

pub enum Marks{
    Marker,
    Modifier,
    Name,
    DataType,
    Operator,
}

pub struct MarkAlphabet{
    order: HashMap::<u32, Marks>,
    count: u32,
}

impl MarkAlphabet{
    pub fn new() -> MarkAlphabet{
        let max: u32 = 5;
        MarkAlphabet{
            order: MarkAlphabet::get_order(max),
            count: 0,
        }
    }

    fn get_order(max: u32) -> HashMap::<u32, Marks>{
        let mut map: HashMap = HashMap::new();
        for i in 0..max{
            match i{
                0 => map.insert(i, Marks::DataType),
                1 => map.insert(i, Marks::Marker),
                2 => map.insert(i, Marks::Modifier),
                3 => map.insert(i, Marks::Name,),
                4 => map.insert(i, Marks::Operator),
            };
        }

        map
    }

    fn get_from_map(&self, index: u32) -> Result::<&Marks, u32>{
        self.order.get(&index).ok_or(self.max)
    }

}

impl Iterator for MarkAlphabet{
    type Item = &Marks;

    fn next(&mut self) -> Option::<Self::Item>{
        let mark_alpha: Option::<&Marks> = self.get_from_map(self.count).ok();
        if let Option::None = mark_alpha{
            self.count = 0;
        }else{
            self.count += 1;
        }

        mark_alpha
    }
}