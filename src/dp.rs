use std::collections::HashMap;

pub struct DPArray<T, const N: usize>([T; N]);

pub struct DPArray2<T, const N: usize, const M: usize>([[T; M]; N]);

pub struct DPMap<K, V, const N: usize>(HashMap<K, V>);
