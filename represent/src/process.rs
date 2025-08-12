
pub trait Calculate_L_R{
    type Item;
    type Key;
    type Value;
    //Can combine itself into a new Data
    type Data;
    type Output;

    fn set_element(el: &Self::Item) -> Self::Value;
    fn element_to_data(operator: &mut Self, key: &Self::Key, element: &Self::Value) -> Self::Data;
    fn size(&self) -> usize;
    fn set_data(operator: &mut Self, data: Self::Data) -> Result::<(), ()>;
    fn calculate(&mut self);
    fn get_output(&self) -> Option::<Self::Output>;
}