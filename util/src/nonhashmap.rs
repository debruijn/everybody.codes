use itertools::{izip, Itertools};
use std::fmt;
use std::fmt::Debug;
use std::iter::Zip;
use std::slice::Iter;

#[derive(Clone, Default)]
pub struct NonHashMapMultiVec<K, V> {
    vec_k: Vec<K>,
    vec_v: Vec<V>,
}

impl<K: Default, V: Default> NonHashMapMultiVec<K, V> {
    pub fn new() -> Self {
        Default::default()
    }
}

impl<K: PartialEq, V> NonHashMapMultiVec<K, V> {
    pub fn insert(&mut self, k: K, v: V) {
        let loc_k = self.vec_k.iter().position(|x| *x == k);
        match loc_k {
            None => {
                self.vec_k.push(k);
                self.vec_v.push(v);
            }
            Some(loc) => {
                self.vec_v[loc] = v;
            }
        }
    }
    pub fn contains_key(&self, k: &K) -> bool {
        let loc_k = self.vec_k.iter().position(|x| *x == *k);
        match loc_k {
            None => false,
            Some(_) => true,
        }
    }

    pub fn get(&self, k: &K) -> Option<&V> {
        let loc_k = self.vec_k.iter().position(|x| *x == *k);
        match loc_k {
            None => None,
            Some(loc) => self.vec_v.get(loc),
        }
    }

    pub fn remove(&mut self, k: &K) -> Option<V> {
        let loc_k = self.vec_k.iter().position(|x| *x == *k);
        match loc_k {
            None => None,
            Some(loc) => {
                self.vec_k.remove(loc);
                Some(self.vec_v.remove(loc))
            }
        }
    }

    pub fn swap_remove(&mut self, k: &K) -> Option<V> {
        let loc_k = self.vec_k.iter().position(|x| *x == *k);
        match loc_k {
            None => None,
            Some(loc) => {
                self.vec_k.swap_remove(loc);
                Some(self.vec_v.swap_remove(loc))
            }
        }
    }
}

impl<K, V> NonHashMapMultiVec<K, V> {
    pub fn iter(&self) -> Zip<Iter<'_, K>, Iter<'_, V>> {
        izip!(self.vec_k.iter(), self.vec_v.iter())
    }

    pub fn len(&self) -> usize {
        self.vec_k.len()
    }

    pub fn values(&self) -> &Vec<V> {
        &self.vec_v
    }

    pub fn keys(&self) -> &Vec<K> {
        &self.vec_k
    }

    pub fn is_empty(&self) -> bool {
        self.vec_k.is_empty()
    }

}

impl<K, V> Debug for NonHashMapMultiVec<K, V>
where
    K: Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

#[derive(Clone, Default)]
pub struct NonHashMapVecTuple<K, V> {
    vec: Vec<(K, V)>,
}

impl<K: Default, V: Default> NonHashMapVecTuple<K, V> {
    pub fn new() -> Self {
        Default::default()
    }
}

impl<K: PartialEq, V> NonHashMapVecTuple<K, V> {
    pub fn insert(&mut self, k: K, v: V) {
        let loc_k = self.vec.iter().position(|x| x.0 == k);
        match loc_k {
            None => {
                self.vec.push((k, v));
            }
            Some(loc) => {
                self.vec[loc] = (k, v);
            }
        }
    }
    pub fn contains_key(&self, k: &K) -> bool {
        let loc = self.vec.iter().position(|x| x.0 == *k);
        match loc {
            None => false,
            Some(_) => true,
        }
    }

    pub fn get(&self, k: &K) -> Option<&V> {
        let loc = self.vec.iter().position(|x| x.0 == *k);
        match loc {
            None => None,
            Some(loc) => Some(&self.vec.get(loc).unwrap().1),
        }
    }

    pub fn remove(&mut self, k: &K) -> Option<V> {
        let loc = self.vec.iter().position(|x| x.0 == *k);
        match loc {
            None => None,
            Some(loc) => Some(self.vec.remove(loc).1),
        }
    }

    pub fn swap_remove(&mut self, k: &K) -> Option<V> {
        let loc = self.vec.iter().position(|x| x.0 == *k);
        match loc {
            None => None,
            Some(loc) => Some(self.vec.swap_remove(loc).1),
        }
    }
}

impl<K, V> NonHashMapVecTuple<K, V> {
    pub fn iter(&self) -> Iter<'_, (K, V)> {
        self.vec.iter()
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn values(&self) -> Vec<&V> {
        self.vec.iter().map(|x| &x.1).collect_vec()
    }

    pub fn keys(&self) -> Vec<&K> {
        self.vec.iter().map(|x| &x.0).collect_vec()
    }

    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

}

impl<K, V> Debug for NonHashMapVecTuple<K, V>
where
    K: Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map()
            .entries(self.vec.iter().map(|&(ref k, ref v)| (k, v)))
            .finish()
    }
}

#[test]
fn try_stuff_out() {
    use std::time::Instant;
    type NonHashMap<K, V> = NonHashMapMultiVec<K, V>;

    let mut nhm = NonHashMapMultiVec::new();
    println!("{:?}", nhm);
    nhm.insert(0.1, 0.1);
    println!("{:?}", nhm);

    println!("{:?}, {:?}", nhm.get(&0.1), nhm.get(&0.2));
    println!("{:?}, {:?}", nhm.contains_key(&0.1), nhm.contains_key(&0.2));

    let mut nhm = NonHashMapMultiVec::new();
    nhm.insert(0.1, "blue");
    nhm.insert(1.2, "green");
    nhm.insert(0.4, "red");
    nhm.insert(0.7, "yellow");
    nhm.insert(-2.3, "orange");
    println!("{}, {:?}", nhm.len(), nhm);
    nhm.remove(&0.4);
    println!("{}, {:?}", nhm.len(), nhm.iter().collect_vec().iter());
    nhm.swap_remove(&0.1);
    println!("{}, {:?}", nhm.len(), nhm);

    let mut nhm = NonHashMapVecTuple::new();
    println!("{:?}", nhm);
    nhm.insert(0.1, 0.1);
    println!("{:?}", nhm);

    println!("{:?}, {:?}", nhm.get(&0.1), nhm.get(&0.2));
    println!("{:?}, {:?}", nhm.contains_key(&0.1), nhm.contains_key(&0.2));

    let mut nhm = NonHashMapVecTuple::new();
    nhm.insert(0.1, "blue");
    nhm.insert(1.2, "green");
    nhm.insert(0.4, "red");
    nhm.insert(0.7, "yellow");
    nhm.insert(-2.3, "orange");
    println!("{}, {:?}", nhm.len(), nhm);
    nhm.remove(&0.4);
    println!("{}, {:?}", nhm.len(), nhm.iter());
    nhm.swap_remove(&0.1);
    println!("{}, {:?}", nhm.len(), nhm);

    let mut nhm = NonHashMap::new();
    nhm.insert(0.1, "blue");
    nhm.insert(1.2, "green");
    nhm.insert(0.4, "red");
    nhm.insert(0.7, "yellow");
    nhm.insert(-2.3, "orange");
    println!("{}, {:?}", nhm.len(), nhm);
    nhm.remove(&0.4);
    println!("{}, {:?}", nhm.len(), nhm.iter().collect_vec().iter());
    nhm.swap_remove(&0.1);
    println!("{}, {:?}", nhm.len(), nhm);

    let r = 10000isize;

    let before = Instant::now();
    let mut nhm1 = NonHashMapMultiVec::new();
    for i in 0..r {
        nhm1.insert(i, i);
    }
    let res = nhm1.iter().map(|x| x.1).sum::<isize>();
    let after = Instant::now();
    println!("{:?} in {:?}", res, after - before);
    let before = Instant::now();
    let mut nhm2 = NonHashMapVecTuple::new();
    for i in 0..r {
        nhm2.insert(i, i);
    }
    let res = nhm2.iter().map(|x| x.1).sum::<isize>();
    let after = Instant::now();
    println!("{:?} in {:?}", res, after - before);

    let before = Instant::now();
    let mut nhm1 = NonHashMapMultiVec::new();
    for i in 0..r {
        nhm1.insert(i, i);
    }
    let res = nhm1.values().into_iter().sum::<isize>();
    let after = Instant::now();
    println!("{:?} in {:?}", res, after - before);
    let before = Instant::now();
    let mut nhm2 = NonHashMapVecTuple::new();
    for i in 0..r {
        nhm2.insert(i, i);
    }
    let res = nhm2.values().into_iter().sum::<isize>();
    let after = Instant::now();
    println!("{:?} in {:?}", res, after - before);
}
