/*
    A Bloom filter is a space-efficient probabilistic data structure,
    conceived by Burton Howard Bloom in 1970, that is used to test whether an element is a member of a set.
    The idea behind a bloom filter is very simple. It is a bit array M of m bits, all set to 0.
    We choose k hash functions, h1,h2,..., hk each with range 1 to m
    When we want to add an element x to the set, we apply each hash function to x, obtaining k values h1(x), h2(x), ..., hk(x),
    which will correspond the positions of the k positive bits in the bit array,
    lets call this bit array X and we xor it with M, which becomes the new M.
    When we want to test if x in i the set, we do M & X, if the result is X, then x is probably in the set, otherwise it is not.
*/

extern crate bit_vec;

use bit_vec::BitVec;
use std::collections::hash_map::{DefaultHasher, RandomState};
use std::hash::{BuildHasher, Hash, Hasher};
use std::marker::PhantomData;

pub struct StandardBloomFilter<T: ?Sized> {
    bitmap: BitVec,
    optimal_m: usize,
    optimal_k: u32,
    hashers: [DefaultHasher; 2],
    _marker: PhantomData<T>,
}

impl<T: ?Sized> StandardBloomFilter<T> {
    pub fn new(items_count: usize, fp_rate: f64) -> Self {
        let optimal_m = Self::bitmap_size(items_count, fp_rate);
        let optimal_k = Self::optimal_k(fp_rate);

        let hashers = [
            RandomState::new().build_hasher(),
            RandomState::new().build_hasher(),
        ];

        StandardBloomFilter {
            bitmap: BitVec::from_elem(optimal_m as usize, false),
            optimal_m,
            optimal_k,
            hashers,
            _marker: PhantomData,
        }
    }

    fn bitmap_size(items_count: usize, fp_rate: f64) -> usize {
        let ln2_2 = core::f64::consts::LN_2 * core::f64::consts::LN_2;
        ((-1.0f64 * items_count as f64 * fp_rate.ln()) / ln2_2).ceil() as usize
    }

    fn optimal_k(fp_rate: f64) -> u32 {
        ((-1.0f64 * fp_rate.ln()) / core::f64::consts::LN_2).ceil() as u32
    }

    pub fn insert(&mut self, item: &T)
    where
        T: Hash,
    {
        let (h1, h2) = self.hash_kernel(item);

        for k_i in 0..self.optimal_k {
            let index = self.get_index(h1, h2, k_i as u64);
            self.bitmap.set(index, true)
        }
    }

    pub fn contains(&mut self, item: &T) -> bool
    where
        T: Hash,
    {
        let (h1, h2) = self.hash_kernel(item);

        for k_i in 0..self.optimal_k {
            let index = self.get_index(h1, h2, k_i as u64);
            if !self.bitmap.get(index).unwrap() {
                return false;
            }
        }
        true
    }

    fn hash_kernel(&self, item: &T) -> (u64, u64)
    where
        T: Hash,
    {
        let hasher1 = &mut self.hashers[0].clone();
        let hasher2 = &mut self.hashers[1].clone();

        item.hash(hasher1);
        item.hash(hasher2);

        let hash1 = hasher1.finish();
        let hash2 = hasher2.finish();

        (hash1, hash2)
    }

    fn get_index(&self, h1: u64, h2: u64, k_i: u64) -> usize {
        (h1.wrapping_add((k_i).wrapping_mul(h2)) % self.optimal_m as u64) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert() {
        let mut bloom = StandardBloomFilter::new(100, 0.01);
        bloom.insert("item");
        assert!(bloom.contains("item"));
    }

    #[test]
    fn check_and_insert() {
        let mut bloom = StandardBloomFilter::new(100, 0.01);
        assert!(!bloom.contains("item_1"));
        assert!(!bloom.contains("item_2"));
        bloom.insert("item_1");
        assert!(bloom.contains("item_1"));
    }
}
