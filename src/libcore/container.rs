// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Container traits

use option::Option;

pub trait Container {
    /// Return the number of elements in the container
    pure fn len(&self) -> uint;

    /// Return true if the container contains no elements
    pure fn is_empty(&self) -> bool;
}

pub trait Mutable: Container {
    /// Clear the container, removing all values.
    fn clear(&mut self);
}

pub trait Map<K, V>: Mutable {
    /// Return true if the map contains a value for the specified key
    pure fn contains_key(&self, key: &K) -> bool;

    /// Visit all keys
    pure fn each_key(&self, f: fn(&K) -> bool);

    /// Visit all values
    pure fn each_value(&self, f: fn(&V) -> bool);

    /// Return the value corresponding to the key in the map
    pure fn find(&self, key: &K) -> Option<&self/V>;

    /// Insert a key-value pair into the map. An existing value for a
    /// key is replaced by the new value. Return true if the key did
    /// not already exist in the map.
    fn insert(&mut self, key: K, value: V) -> bool;

    /// Remove a key-value pair from the map. Return true if the key
    /// was present in the map, otherwise false.
    fn remove(&mut self, key: &K) -> bool;
}

pub trait Set<T>: Mutable {
    /// Return true if the set contains a value
    pure fn contains(&self, value: &T) -> bool;

    /// Add a value to the set. Return true if the value was not already
    /// present in the set.
    fn insert(&mut self, value: T) -> bool;

    /// Remove a value from the set. Return true if the value was
    /// present in the set.
    fn remove(&mut self, value: &T) -> bool;

    /// Return true if the set has no elements in common with `other`.
    /// This is equivalent to checking for an empty intersection.
    pure fn is_disjoint(&self, other: &Self) -> bool;

    /// Return true if the set is a subset of another
    pure fn is_subset(&self, other: &Self) -> bool;

    /// Return true if the set is a superset of another
    pure fn is_superset(&self, other: &Self) -> bool;

    /// Visit the values representing the difference
    pure fn difference(&self, other: &Self, f: fn(&T) -> bool);

    /// Visit the values representing the symmetric difference
    pure fn symmetric_difference(&self, other: &Self, f: fn(&T) -> bool);

    /// Visit the values representing the intersection
    pure fn intersection(&self, other: &Self, f: fn(&T) -> bool);

    /// Visit the values representing the union
    pure fn union(&self, other: &Self, f: fn(&T) -> bool);
}
