//! Defining all native types (and functions?)

use crate::compiler_types::*;
use crate::levelstring::*;
//use std::collections::HashMap;
use crate::compiler::RuntimeError;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Group {
    pub id: u16,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]

pub struct Color {
    pub id: u16,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Block {
    pub id: u16,
}

impl Block {}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Item {
    pub id: u16,
}

pub fn context_trigger(context: Context, _globals: &mut Globals, info: CompilerInfo) -> GDObj {
    GDObj {
        obj_id: 0,
        groups: vec![context.start_group],
        target: Group { id: 0 },
        spawn_triggered: context.spawn_triggered,
        params: Vec::new(),
        func_id: info.func_id,
    }
}

const TYPE_MEMBER_NAME: &str = "TYPE";
impl Value {
    pub fn member(
        &self,
        member: String,
        context: &Context,
        globals: &mut Globals,
        _: CompilerInfo,
    ) -> Option<Value> {
        //println!("{:?}", context.implementations);
        let get_impl = |t: u16, m: String| match context.implementations.get(&(t)) {
            Some(imp) => match imp.get(&m) {
                Some(mem) => Some((*globals).stored_values[*mem as usize].clone()),
                None => None,
            },
            None => None,
        };
        if member == TYPE_MEMBER_NAME {
            Some(Value::TypeIndicator(match self {
                Value::Dict(dict) => match dict.get(TYPE_MEMBER_NAME) {
                    Some(value) => match (*globals).stored_values[*value as usize].clone() {
                        Value::TypeIndicator(s) => s,
                        _ => unreachable!(),
                    },
                    None => TypeID::from(self),
                },

                _ => TypeID::from(self),
            }))
        } else {
            match self {
                Value::Func(f) => {
                    if member == "group" {
                        return Some(Value::Group(f.start_group));
                    }
                }

                Value::Str(a) => {
                    if member == "length" {
                        return Some(Value::Number(a.len() as f64));
                    }
                }
                Value::Array(a) => {
                    if member == "length" {
                        return Some(Value::Number(a.len() as f64));
                    }
                }
                _ => (),
            };

            let my_type = TypeID::from(self);
            match self {
                Value::Builtins => Some(Value::BuiltinFunction(member)),
                Value::Dict(dict) => match dict.get(&member) {
                    Some(value) => Some((*globals).stored_values[*value as usize].clone()),
                    None => get_impl(my_type, member).clone(),
                },
                Value::Func(f) => {
                    if &member == "start_group" {
                        Some(Value::Group(f.start_group))
                    } else {
                        get_impl(my_type, member).clone()
                    }
                }
                _ => get_impl(my_type, member).clone(),
            }
        }
    }
}

pub fn built_in_function(
    name: &str,
    arguments: Vec<Value>,
    info: CompilerInfo,
    globals: &mut Globals,
    context: Context,
) -> Result<Value, RuntimeError> {
    Ok(match name {
        "print" => {
            let mut out = String::new();
            for val in arguments {
                out += &val.to_str(globals);
            }
            //out.pop();
            println!("{}", out);
            Value::Null
        }

        "sin" | "cos" | "tan" | "asin" | "acos" | "atan" | "floor" | "ceil" => {
            if arguments.len() != 1 {
                return Err(RuntimeError::BuiltinError {
                    message: "Expected one error".to_string(),
                    pos: (0, 0),
                });
            }

            match &arguments[0] {
                Value::Number(n) => Value::Number(match name {
                    "sin" => n.sin(),
                    "cos" => n.cos(),
                    "tan" => n.tan(),
                    "asin" => n.asin(),
                    "acos" => n.acos(),
                    "atan" => n.atan(),
                    "floor" => n.floor(),
                    "ceil" => n.ceil(),

                    _ => unreachable!(),
                }),

                a => {
                    return Err(RuntimeError::BuiltinError {
                        message: format!("Expected number, found {}", a.to_str(globals)),
                        pos: (0, 0),
                    })
                }
            }
        }

        "add" => {
            if arguments.len() != 1 {
                return Err(RuntimeError::BuiltinError {
                    message: "Expected one error".to_string(),
                    pos: (0, 0),
                });
            }

            match &arguments[0] {
                Value::Obj(obj) => {
                    let c_t = context_trigger(context.clone(), globals, info.clone());
                    (*globals).func_ids[info.func_id].obj_list.push(
                        GDObj {
                            params: obj.clone(),
                            groups: vec![context.start_group],
                            ..c_t
                        }
                        .context_parameters(context.clone()),
                    );
                }

                a => {
                    return Err(RuntimeError::BuiltinError {
                        message: format!("Expected object, found {}", a.to_str(globals)),
                        pos: (0, 0),
                    })
                }
            }

            Value::Null
        }

        "current_context" => Value::Str(format!("{:?}", context)),

        a => {
            return Err(RuntimeError::RuntimeError {
                message: format!("Nonexistant builtin-function: {}", a),
                pos: (0, 0),
            })
        }
    })
}
