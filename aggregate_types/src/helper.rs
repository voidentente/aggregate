use crate::*;

use std::collections::hash_map;
use std::iter::IntoIterator;
use std::ops::Index;
use std::slice;
use std::vec;

impl IntoIterator for Fields {
    type Item = (String, Field);
    type IntoIter = hash_map::IntoIter<String, Field>;

    #[inline]
    #[must_use]
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Fields {
    type Item = (&'a String, &'a Field);
    type IntoIter = hash_map::Iter<'a, String, Field>;

    #[inline]
    #[must_use]
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl Fields {
    #[inline]
    #[must_use]
    pub fn get(&self, k: &str) -> Option<&Field> {
        self.0.get(k)
    }
}

impl IntoIterator for Attributes {
    type Item = Attribute;
    type IntoIter = vec::IntoIter<Attribute>;

    #[inline]
    #[must_use]
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Attributes {
    type Item = &'a Attribute;
    type IntoIter = slice::Iter<'a, Attribute>;

    #[inline]
    #[must_use]
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl Index<usize> for Attributes {
    type Output = Attribute;

    #[inline]
    #[must_use]
    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}

impl Attributes {
    #[inline]
    #[must_use]
    pub fn get(&self, index: usize) -> Option<&Attribute> {
        self.0.get(index)
    }
}
