use bevy::prelude::PartialReflect;
use bevy_mod_scripting::core::bindings::{ScriptValue, WorldGuard};
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

#[derive(Debug)]
pub struct MissingArgsError(String);

impl std::fmt::Display for MissingArgsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Missing args: {}", self.0))
    }
}

impl std::error::Error for MissingArgsError {}

pub trait GetArgs {
    fn get_bool(&self, name: &str) -> Option<bool>;

    fn get_f32(&self, name: &str) -> Option<f32>;

    fn get_f64(&self, name: &str) -> Option<f64>;

    fn get_i64(&self, name: &str) -> Option<i64>;

    fn get_usize(&self, name: &str) -> Option<usize>;

    fn get_string(&self, name: &str) -> Option<String>;

    fn get_reflect<T: Clone + PartialReflect>(&self, name: &str, world: WorldGuard) -> Option<T>;
}

impl GetArgs for HashMap<String, ScriptValue> {
    fn get_bool(&self, name: &str) -> Option<bool> {
        self.get(name).and_then(|v| match v {
            ScriptValue::Bool(b) => Some(*b),
            _ => None,
        })
    }

    fn get_f32(&self, name: &str) -> Option<f32> {
        self.get(name).and_then(|v| match v {
            ScriptValue::Float(f) => Some(*f as f32),
            ScriptValue::Integer(i) => Some(*i as f32),
            _ => None,
        })
    }

    fn get_f64(&self, name: &str) -> Option<f64> {
        self.get(name).and_then(|v| match v {
            ScriptValue::Float(f) => Some(*f),
            ScriptValue::Integer(i) => Some(*i as f64),
            _ => None,
        })
    }

    fn get_i64(&self, name: &str) -> Option<i64> {
        self.get(name).and_then(|v| match v {
            ScriptValue::Integer(i) => Some(*i),
            ScriptValue::Float(f) => Some(*f as i64),
            _ => None,
        })
    }

    fn get_usize(&self, name: &str) -> Option<usize> {
        self.get(name).and_then(|v| match v {
            ScriptValue::Integer(i) => Some(*i as usize),
            ScriptValue::Float(f) => Some(*f as usize),
            _ => None,
        })
    }

    fn get_string(&self, name: &str) -> Option<String> {
        self.get(name)
            .and_then(|v| v.clone().as_string().ok())
            .map(|s| s.into_owned())
    }

    fn get_reflect<T: Clone + PartialReflect>(&self, name: &str, world: WorldGuard) -> Option<T> {
        let v = self.get(name)?;
        match v {
            ScriptValue::Reference(r) => r.downcast(world).ok(),
            _ => None,
        }
    }
}
