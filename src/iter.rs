use std::cell::RefCell;

pub trait BuildIter {
    type Item;
    type FilterItem;

    ///
    /// Obtiene los datos remotos y devuelve un listado
    ///
    fn get_data_from_remote(&self, filter: &Self::FilterItem) -> Vec<Self::Item>;
    ///
    /// Crea un recurso a partir del dato pasado por parámetro
    ///
    fn update_item(&self, data: &Self::Item) -> Self::Item;
    ///
    /// Actualiza el offset
    ///
    fn update_filter_offset(&self, filter: &Self::FilterItem) -> Self::FilterItem;
}

#[derive(Debug, Clone)]
pub struct MalchimpIter<'a, B>
where
    B: BuildIter,
{
    pub builder: &'a B,
    pub data: RefCell<Vec<B::Item>>,
    pub cur_filters: B::FilterItem,
    pub cur_it: usize,
    pub total_items: u32,
}

impl<'a, B> Iterator for MalchimpIter<'a, B>
where
    B: BuildIter,
{
    type Item = B::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let mut in_data = self.data.borrow_mut();

        if self.cur_it == in_data.len() - 2 && self.cur_it < (self.total_items as usize) {
            let new_filter = self.builder.update_filter_offset(&self.cur_filters);
            let list_of_types = self.builder.get_data_from_remote(&new_filter);
            self.cur_filters = new_filter;

            for r in list_of_types {
                in_data.push(r);
            }
        }

        if self.cur_it < in_data.len() {
            let data = &in_data[self.cur_it];
            self.cur_it += 1;
            return Some(self.builder.update_item(data));
        }

        None
    }
}
