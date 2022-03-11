

const CAPACITY: u64 = 16;
const FACTOR: f64 = 0.75;
const KNUTH:u64 = 2654435769;
const MOST_SIG: u64 = 16;
type Collide = (u64, u64);

#[derive(Clone,Debug)]
pub struct Bucket
{
    key: u64,
    value: u64,
    index: u64,
    collisions: Vec<u64>,
}


impl Bucket {


    pub fn new(key: u64, value: u64, index: u64) -> Self  {
        Bucket {
            key,
            value,
            index,
            collisions : vec![],
        }
    }

    pub fn key_ref(&self) -> &u64 {
        &self.key
    }
    pub fn value_ref(&self) -> &u64 {
        &self.value
    }
    pub fn index_ref(&self) -> &u64 {
        &self.index
    }
    pub fn value_mut(&mut self) -> &mut u64 {
        &mut self.value
    }
    pub fn index_mut(&mut self) -> &mut u64 {
        &mut self.index
    }
    pub fn key(self) -> u64 {
        self.key
    }
    pub fn value(self) -> u64 {
        self.value
    }
    pub fn key_value(self) -> (u64, u64) {
        (self.key, self.value)
    }
    pub fn refs(&self) -> (&u64, &u64) {
        (&self.key, &self.value)
    }
    pub fn ref_mut(&mut self) -> (&u64, &mut u64) {
        (&self.key, &mut self.value)
    }
    pub fn muts(&mut self) -> (&mut u64, &mut u64) {
        (&mut self.key, &mut self.value)
    }
    pub fn contains_in_collisions(&self , item : &u64) -> bool {
        self.collisions.contains(item)
    }
    pub fn add_to_collisions(&mut self, key : &u64) {
        self.collisions.push(*key);
    }
    pub  fn update_index (&mut self , item : &u64){
        self.index += item;
    }
}









#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
