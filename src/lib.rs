

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

#[derive(Clone,Debug)]
pub struct Table {
    cells: Vec<Option<Bucket>>,
    indexes: Vec<u64>,
    capacity: u64,
    count: u64,

}

impl Table {
    pub fn new() -> Self {
        Table {
            cells: vec![None; CAPACITY as usize],
            indexes: vec![0; CAPACITY as usize],
            capacity: CAPACITY,
            count: 0,
        }
    }

    pub fn new_grow(capacity:&u64) -> Self {
        Table {
            cells: vec![None; *capacity as usize],
            indexes: vec![0; *capacity as usize],
            capacity: *capacity,
            count: 0,
        }
    }

    pub fn insert(&mut self, key: u64, value: u64) {
        if (self.count + 1) as f64 > self.capacity as f64 * FACTOR {
            self.grow_capacity(&key, &value);
        }
        let ind = calculate_fxhash(&key) & (self.capacity - 1);
        let mut buc = Bucket::new(key, value, ind);
        match self.cells[ind as usize].is_none() {
            true => {
                self.count += 1;
                self.cells[ind as usize] = Some(buc);
                let index = knuth_multiplicative_hash(key) & (self.indexes.len() as u64 -1);
                self.indexes[index as usize] =  ind;
                println!("knuth_index is :{:?}", self.indexes);
                return;
            },
            _ => {
                self.resolve_collision(&mut buc, self.cells[ind as usize].clone().unwrap().key_ref());
            }
        }
    }

    pub fn get(&mut self , key:u64)-> u64{
        let index = knuth_multiplicative_hash(key) & (self.indexes.len() as u64 -1);
        println!("knuth_index : {:?}",index);
        self.cells[self.indexes[index as usize] as usize].clone().unwrap().value()
    }

    pub fn contains_in_collisions(&self, key: &u64) -> bool {
        todo!()
    }

    fn resolve_collision(&mut self, buc: &mut Bucket, key: &u64) {
        let mut q: Queue<Collide> = queue![];
        let mut collision: Collide = (0, 0);
        collision.0 = buc.key;
        collision.1 = *key;
        q.add(collision);
        while q.size() > 0  {
            let  coll: Collide = q.peek().unwrap();
            q.remove();
            let  index_from_key1 = calculate_fxhash(&coll.0) & (self.capacity - 1);
            println!("col first = {:?}",coll);
            let index_from_key2 = calculate_fxhash(&coll.1) & (self.capacity - 1);
            let key_ind = calculate_fxhash(&coll.0) & (3);
            println!("key_ind {:?}  ", key_ind);
            let cell_ind = calculate_fnvhash(&coll.1) & (3);
            println!("cell_ind {:?}  ", cell_ind);
            self.calc_index(index_from_key1 + key_ind, &coll.0 , &buc.value, &mut q);
            //self.calc_index(index_from_key2 + cell_ind, &coll.1, &buc.value, &mut q);
            self.get_bucket_asmut(&buc.index).add_to_collisions(&coll.0);
            self.get_bucket_asmut(&buc.index).add_to_collisions(&coll.1);
            //self.add_to_bucks(&buc.key, &key, & buc.value, coll, &mut q);
            //self.cells[coll.0 as usize].clone().unwrap().update_index(& key_ind);
            //self.cells[coll.1 as usize].clone().unwrap().update_index(& cell_ind);

        }
    }

    fn calc_index(&mut self, key_index :u64, key : &u64, value :&u64, q:&mut Queue<Collide> ){
        match key_index < self.capacity {
            true => {
                match self.cells[key_index as usize].is_none() {
                    true => {
                        self.count += 1;
                        let buc = Bucket::new(*key, *value, key_index);
                        println!("buc is : {:?}",buc);
                        self.cells[key_index as usize] = Some(buc);
                        let index = knuth_multiplicative_hash(*key) & (self.indexes.len() as u64 -1);
                        println!("knuth_index is :{:?}", index);
                        self.indexes[index as usize] = key_index;
                        println!("key_index is :{:?}",key_index);
                    }
                    _ => {
                        let collision: Collide = (*key,self.cells[key_index as usize].clone().unwrap().key() );
                        self.grow_capacity(&key,&value);
                        //q.add(collision);
                        //self.cells[(key_index as usize)] = None;
                        println!("keys in collision : {:?}-{:?}",self.cells[key_index as usize].clone().unwrap().key(),*key);
                        println!("collision is : {:?}",collision);
                        println!("collision key index is : {:?}",key_index);
                        println!("queue index is : {:?}",q);
                    }

                }

            }
            _ =>{
                self.grow_capacity(&key,&value);
                println!("col is : {:?}  ", q);
            }
        }
    }

    fn grow_capacity(  &mut self, key: &u64, val: &u64) {
        println!("in grow_capacity  ");
        let new_capacity = next_power_of2(&self.capacity);
        println!("in grow_capacity new capacity {:?}  ",new_capacity);
        let temp_old = self.cells.clone();
        //let temp_new =  Table::new_grow(&new_capacity);
        self.cells = vec![None; new_capacity as usize];
        self.indexes = vec![0; new_capacity as usize];
        self.count = 0;
        self.capacity = new_capacity;
        for x in temp_old.into_iter() {
            match x {
                None => (),
                Some(x) =>{
                    self.insert(x.key,x.value);
                }
            }

        }
        self.insert(*key,*val);
    }

    fn get_bucket_asmut(&mut self , index :&u64) ->  &mut Bucket {
        let x : &mut Bucket =  self.cells[*index as usize].as_mut().unwrap();
        x
    }


}


#[inline]
fn knuth_multiplicative_hash (key:u64)-> u64{
    (key * KNUTH) >> (32 - MOST_SIG)
}



#[inline]
fn calculate_fxhash(key_to_hash: &u64) -> u64 {
    let mut state = FxHasher64::default();
    state.write_u64(*key_to_hash);
    state.finish() as u64
}

#[inline]
fn calculate_ahash(key_to_hash: &u64) -> u64 {
    let mut state = AHasher::default();
    state.write_u64(*key_to_hash);
    state.finish() as u64
}

#[inline]
fn calculate_fnvhash(key_to_hash: &u64) -> u64 {
    let mut state = FnvHasher::default();
    state.write_u64(*key_to_hash);
    state.finish() as u64
}

#[inline]
fn next_power_of2(n: &u64) -> u64 {
    let mut p: u64 = 1;
    while p <= *n {
        p <<= 1;
    }
    p
}





#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
