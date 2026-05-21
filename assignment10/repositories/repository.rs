pub trait Repository<T, ID> {
    fn save(&mut self, entity: T);

    fn find_by_id(&self, id: &ID) -> Option<&T>;

    fn find_all(&self) -> Vec<&T>;

    fn delete(&mut self, id: &ID);
}
