///types and functions used by the compiler
use crate::builtin::*;

use crate::compiler_info::CodeArea;

use crate::globals::Globals;
use crate::value::*;

use core::panic;
use std::collections::HashMap;

use crate::compiler::{BUILTIN_STORAGE, NULL_STORAGE};

pub type StoredValue = u32; //index to stored value in globals.stored_values

#[derive(Debug)]
pub struct ValStorage {
    pub map: HashMap<StoredValue, StoredValData>,
    pub preserved_stack: Vec<Vec<StoredValue>>,
    pub prev_value_count: u32,
}

#[derive(Debug, Clone)]
pub struct StoredValData {
    pub val: Value,
    pub fn_context: Group,
    pub mutable: bool,
    pub def_area: CodeArea,
    marked: bool,
}
// LIFETIME:
//
// value gets deleted when lifetime reaches 0
// deeper scope => lifetime++
// shallower scope => lifetime--

impl std::ops::Index<StoredValue> for ValStorage {
    type Output = Value;

    fn index(&self, i: StoredValue) -> &Self::Output {
        &self
            .map
            .get(&i)
            .unwrap_or_else(|| panic!("index {} not found", i))
            .val
    }
}

impl std::ops::IndexMut<StoredValue> for ValStorage {
    fn index_mut(&mut self, i: StoredValue) -> &mut Self::Output {
        &mut self.map.get_mut(&i).unwrap().val
    }
}

impl ValStorage {
    pub fn new() -> Self {
        ValStorage {
            map: vec![
                (
                    BUILTIN_STORAGE,
                    StoredValData {
                        val: Value::Builtins,
                        fn_context: Group::new(0),
                        mutable: false,
                        def_area: CodeArea::new(),
                        marked: false,
                    },
                ),
                (
                    NULL_STORAGE,
                    StoredValData {
                        val: Value::Null,
                        fn_context: Group::new(0),
                        mutable: false,
                        def_area: CodeArea::new(),
                        marked: false,
                    },
                ),
            ]
            .iter()
            .cloned()
            .collect(),
            preserved_stack: Vec::new(),
            prev_value_count: 100,
        }
    }

    pub fn set_mutability(&mut self, index: StoredValue, mutable: bool) {
        if !mutable || !matches!(self[index], Value::Macro(_)) {
            (*self.map.get_mut(&index).unwrap()).mutable = mutable;
        }

        match self[index].clone() {
            Value::Array(a) => {
                for e in a {
                    self.set_mutability(e, mutable);
                }
            }
            Value::Dict(a) => {
                for (_, e) in a {
                    self.set_mutability(e, mutable);
                }
            }
            Value::Macro(_) => (),
            _ => (),
        };
    }

    fn marked(&self, root: StoredValue) -> bool {
        self.map
            .get(&root)
            .unwrap_or_else(|| panic!("Could not find {}", root))
            .marked
    }

    pub fn mark(&mut self, root: StoredValue) {
        if !self.marked(root) {
            (*self.map.get_mut(&root).unwrap()).marked = true;
            match self[root].clone() {
                Value::Array(a) => {
                    for e in a.iter() {
                        self.mark(*e)
                    }
                }
                Value::Dict(a) => {
                    for (_, e) in a.iter() {
                        self.mark(*e)
                    }
                }
                Value::Macro(m) => {
                    for (_, e, _, e2, _) in m.args {
                        if let Some(val) = e {
                            self.mark(val)
                        }
                        if let Some(val) = e2 {
                            self.mark(val)
                        }
                    }

                    for (_, v) in m.def_variables.iter() {
                        self.mark(*v)
                    }
                }
                _ => (),
            };
        }
    }

    pub fn sweep(&mut self) {
        self.map.retain(|_, a| a.marked);
        for v in self.map.values_mut() {
            (*v).marked = false;
        }
    }

    // pub fn clean_up(&mut self) {
    //     self.map.retain(|_, a| {
    //         if let Some(n) = a.lifetime {
    //             n > 0
    //         } else {
    //             true
    //         }
    //     });
    // }

    // pub fn get_lifetime(&self, index: usize) -> u16 {
    //     self.map.get(&index).unwrap().lifetime
    // }
}
// pub fn store_value(
//     val: Value,
//     lifetime: u16,
//     globals: &mut Globals,
//     context: &Context,
//     area: CodeArea,
// ) -> StoredValue {
//     let index = globals.val_id;
//     let mutable = !matches!(val, Value::Macro(_));
//     ////println!("index: {}, value: {}", index, val.to_str(&globals));
//     (*globals).stored_values.map.insert(
//         index,
//         StoredValData {
//             val,
//             fn_context: context.start_group,
//             mutable,
//             lifetime,
//             def_area: area,
//         },
//     );
//     (*globals).val_id += 1;
//     index
// }

pub fn clone_and_get_value(
    index: StoredValue,
    globals: &mut Globals,
    fn_context: Group,
    constant: bool,
) -> Value {
    let mut old_val = globals.stored_values[index].clone();

    match &mut old_val {
        Value::Array(arr) => {
            old_val = Value::Array(
                arr.iter()
                    .map(|x| clone_value_preserve_area(*x, globals, fn_context, constant))
                    .collect(),
            );
        }

        Value::Dict(arr) => {
            old_val = Value::Dict(
                arr.iter()
                    .map(|(k, v)| {
                        (
                            *k,
                            clone_value_preserve_area(*v, globals, fn_context, constant),
                        )
                    })
                    .collect(),
            );
        }

        Value::Macro(m) => {
            for arg in &mut m.args {
                if let Some(def_val) = &mut arg.1 {
                    (*def_val) = clone_value_preserve_area(*def_val, globals, fn_context, constant);
                }

                if let Some(def_val) = &mut arg.3 {
                    (*def_val) = clone_value_preserve_area(*def_val, globals, fn_context, constant);
                }
            }

            // for (_, v) in m.def_context.variables.iter_mut() {
            //     (*v) = clone_value(*v, lifetime, globals, context, constant)
            // }
        }
        _ => (),
    };

    old_val
}
// pub fn get_all_ptrs_used(index: usize, globals: &mut Globals) -> Vec<usize> {
//     let mut out = vec![index];
//     match globals.stored_values[index].clone() {
//         Value::Array(arr) => {
//             for v in arr {
//                 out.extend(get_all_ptrs_used(v, globals))
//             }
//         }

//         Value::Dict(arr) => {
//             for v in arr.values() {
//                 out.extend(get_all_ptrs_used(*v, globals))
//             }
//         }

//         Value::Macro(m) => {
//             for arg in &m.args {
//                 if let Some(def_val) = &arg.1 {
//                     out.extend(get_all_ptrs_used(*def_val, globals));
//                 }

//                 if let Some(def_val) = &arg.3 {
//                     out.extend(get_all_ptrs_used(*def_val, globals));
//                 }
//             }

//             for (_, (v, _)) in m.def_context.variables.iter() {
//                 out.extend(get_all_ptrs_used(*v, globals));
//             }
//         }
//         _ => (),
//     };
//     out
// }

pub fn clone_value(
    index: StoredValue,
    globals: &mut Globals,
    fn_context: Group,
    constant: bool,
    area: CodeArea,
) -> StoredValue {
    let old_val = clone_and_get_value(index, globals, fn_context, constant);

    //clone all inner values
    //do the thing
    //bing bang
    //profit
    let new_index = globals.val_id;
    //println!("1index: {}, value: {}", new_index, old_val.to_str(&globals));

    (*globals).stored_values.map.insert(
        new_index,
        StoredValData {
            val: old_val,
            fn_context,
            mutable: !constant,
            def_area: area,
            marked: false,
        },
    );
    (*globals).val_id += 1;
    new_index
}

pub fn clone_value_preserve_area(
    index: StoredValue,
    globals: &mut Globals,
    fn_context: Group,
    constant: bool,
) -> StoredValue {
    let old_val = clone_and_get_value(index, globals, fn_context, constant);

    //clone all inner values
    //do the thing
    //bing bang
    //profit
    let new_index = globals.val_id;
    ////println!("1index: {}, value: {}", new_index, old_val.to_str(&globals));

    (*globals).stored_values.map.insert(
        new_index,
        StoredValData {
            val: old_val,
            fn_context,
            mutable: !constant,

            def_area: globals.get_area(index),
            marked: false,
        },
    );
    (*globals).val_id += 1;
    new_index
}

// pub fn clone_value_to(
//     index: usize,
//     to: usize,
//     lifetime: u16,
//     globals: &mut Globals,
//     fn_context: Group,
//     constant: bool,
//     area: CodeArea,
// ) {
//     let old_val = clone_and_get_value(index, lifetime, globals, fn_context, constant, area.clone());

//     //clone all inner values
//     //do the thing
//     //bing bang
//     //profit
//     let new_index = to;
//     ////println!("1index: {}, value: {}", new_index, old_val.to_str(&globals));

//     (*globals).stored_values.map.insert(
//         new_index,
//         StoredValData {
//             val: old_val,
//             fn_context,
//             mutable: !constant,
//             lifetime,
//             def_area: area,
//         },
//     );
// }

pub fn store_const_value(
    val: Value,
    globals: &mut Globals,
    fn_context: Group,
    area: CodeArea,
) -> StoredValue {
    let index = globals.val_id;
    // println!(
    //     "2index: {}, value: {}, area: {:?}",
    //     index,
    //     val.to_str(&globals),
    //     area
    // );

    (*globals).stored_values.map.insert(
        index,
        StoredValData {
            val,
            fn_context,
            mutable: false,
            def_area: area,
            marked: false,
        },
    );
    (*globals).val_id += 1;
    index
}

pub fn store_val_m(
    val: Value,
    globals: &mut Globals,
    fn_context: Group,
    constant: bool,
    area: CodeArea,
) -> StoredValue {
    let index = globals.val_id;

    (*globals).stored_values.map.insert(
        index,
        StoredValData {
            val,
            fn_context,
            mutable: !constant,
            def_area: area,
            marked: false,
        },
    );
    (*globals).val_id += 1;
    index
}
