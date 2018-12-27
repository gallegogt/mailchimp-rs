pub trait BuildIter {
    type Item;
    type Resource;
    type FilterItem;

    ///
    /// Obtiene los datos remotos y devuelve un listado
    ///
    fn get_data_from_remote(&self, filter: &Self::FilterItem) -> Vec<Self::Item>;
    ///
    /// Crea un recurso a partir del dato pasado por parámetro
    ///
    fn create_resource(&self, data: &Self::Item) -> Self::Resource;
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
    pub data: Vec<B::Item>,
    pub cur_filters: B::FilterItem,
    pub cur_it: usize,
}

impl <'a, B> Iterator for MalchimpIter<'a, B>
where
    B: BuildIter,
{
    type Item = B::Resource;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur_it == self.data.len() - 2 {
            let new_filter = self.builder.update_filter_offset(&self.cur_filters);
            let list_of_types = self.builder.get_data_from_remote(&new_filter);
            self.cur_filters = new_filter;

            for r in list_of_types {
                self.data.push(r);
            }
        }

        if self.cur_it < self.data.len() {
            let data = &self.data[self.cur_it];
            self.cur_it += 1;
            return Some(self.builder.create_resource(data));
        }

        None
    }
}
