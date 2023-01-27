use crate::*;

use std::collections::hash_map;
use std::iter::IntoIterator;
use std::ops::Index;
use std::slice;
use std::vec;

impl IntoIterator for Fields {
    type Item = (String, Field);
    type IntoIter = hash_map::IntoIter<String, Field>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Fields {
    type Item = (&'a String, &'a Field);
    type IntoIter = hash_map::Iter<'a, String, Field>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl IntoIterator for Attributes {
    type Item = Attribute;
    type IntoIter = vec::IntoIter<Attribute>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Attributes {
    type Item = &'a Attribute;
    type IntoIter = slice::Iter<'a, Attribute>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl Index<usize> for Attributes {
    type Output = Attribute;

    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}
