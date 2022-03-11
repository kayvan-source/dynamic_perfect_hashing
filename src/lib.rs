

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
}









#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
