// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::cell::RefCell;

use builder::Step;

/// This is essentially a HashMap which allows storing any type in its input and
/// any type in its output. It is a write-once cache; values are never evicted,
/// which means that references to the value can safely be returned from the
/// get() method.
//
// FIXME: This type does not permit retrieving &Path from a PathBuf, primarily
// due to a lack of any obvious way to ensure that this is safe, but also not
// penalize other cases (e.g., deserializing u32 -> &u32, which is non-optimal).
#[derive(Debug)]
pub struct Cache(
    RefCell<HashMap<
        TypeId,
        Box<Any>, // actually a HashMap<Step, Step::Output>
    >>
);

impl Cache {
    pub fn new() -> Cache {
        Cache(RefCell::new(HashMap::new()))
    }

    pub fn put<S: Step>(&self, step: S, value: S::Output) {
        let mut cache = self.0.borrow_mut();
        let type_id = TypeId::of::<S>();
        let stepcache = cache.entry(type_id)
                        .or_insert_with(|| Box::new(HashMap::<S, S::Output>::new()))
                        .downcast_mut::<HashMap<S, S::Output>>()
                        .expect("invalid type mapped");
        assert!(!stepcache.contains_key(&step), "processing {:?} a second time", step);
        stepcache.insert(step, value);
    }

    pub fn get<S: Step>(&self, step: &S) -> Option<S::Output> {
        let mut cache = self.0.borrow_mut();
        let type_id = TypeId::of::<S>();
        let stepcache = cache.entry(type_id)
                        .or_insert_with(|| Box::new(HashMap::<S, S::Output>::new()))
                        .downcast_mut::<HashMap<S, S::Output>>()
                        .expect("invalid type mapped");
        stepcache.get(step).map(|o| o.clone())
    }
}
