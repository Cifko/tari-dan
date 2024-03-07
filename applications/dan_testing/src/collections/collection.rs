use std::collections::HashMap;

pub trait Collection<T> {
    fn new() -> Self;
    fn items(&self) -> &HashMap<String, T>;
    fn items_mut(&mut self) -> &mut HashMap<String, T>;
    fn has(&self, id: String) -> bool {
        return self.items().contains_key(&id);
    }
    fn get_any(&self) -> Option<&T> {
        return self.items().values().next();
    }
    fn get_any_mut(&mut self) -> Option<&mut T> {
        return self.items_mut().values_mut().next();
    }
    fn iter(&self) -> std::collections::hash_map::Values<String, T> {
        return self.items().values();
    }
}
