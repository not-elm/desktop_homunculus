use bevy::prelude::{Event, Events, World};
use bevy_mod_scripting::core::bindings::{
    ScriptValue, ThreadWorldContainer, WorldAccessGuard, WorldContainer, WorldGuard,
};
use serde_json::Value;
use std::collections::HashMap;

pub trait WorldEvents {
    fn read_all_events<E: Clone + Event>(&mut self) -> Vec<E>;
}

impl WorldEvents for World {
    fn read_all_events<E: Clone + Event>(&mut self) -> Vec<E> {
        let events = self.resource::<Events<E>>();
        let mut cursor = events.get_cursor();
        cursor.read(events).cloned().collect::<Vec<_>>()
    }
}

pub fn with_static_guard(world: &mut World, f: impl FnOnce()) {
    WorldAccessGuard::with_static_guard(world, move |guard| {
        ThreadWorldContainer.set_world(guard).unwrap();
        f();
    });
}

pub fn with_static_guard_with_guard(world: &mut World, f: impl FnOnce(WorldGuard)) {
    WorldAccessGuard::with_static_guard(world, move |guard| {
        ThreadWorldContainer.set_world(guard.clone()).unwrap();
        f(guard);
    });
}

pub fn json_to_script_value(value: Value) -> ScriptValue {
    match value {
        Value::Null => ScriptValue::Unit,
        Value::Bool(b) => ScriptValue::Bool(b),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                ScriptValue::Integer(i)
            } else if let Some(n) = n.as_u64() {
                ScriptValue::Integer(n as i64)
            } else if let Some(n) = n.as_f64() {
                ScriptValue::Float(n)
            } else if let Some(n) = n.as_u128() {
                ScriptValue::Integer(n as i64)
            } else {
                ScriptValue::Unit
            }
        }
        Value::String(s) => ScriptValue::String(s.into()),
        Value::Array(values) => {
            let mut vec = Vec::new();
            for value in values {
                vec.push(json_to_script_value(value));
            }
            ScriptValue::List(vec)
        }
        Value::Object(map) => {
            let mut table = HashMap::new();
            for (key, value) in map {
                let value = json_to_script_value(value);
                table.insert(key, value);
            }
            ScriptValue::Map(table)
        }
    }
}

pub fn script_value_to_json(value: &ScriptValue) -> Value {
    match value {
        ScriptValue::Unit => Value::Null,
        ScriptValue::Bool(b) => Value::Bool(*b),
        ScriptValue::Integer(i) => Value::Number((*i).into()),
        ScriptValue::Float(f) => Value::Number(serde_json::Number::from_f64(*f).unwrap()),
        ScriptValue::String(s) => Value::String(s.to_string()),
        ScriptValue::List(vec) => {
            let values: Vec<Value> = vec.iter().map(script_value_to_json).collect();
            Value::Array(values)
        }
        ScriptValue::Map(map) => {
            let mut obj = serde_json::Map::new();
            for (key, value) in map {
                obj.insert(key.clone(), script_value_to_json(value));
            }
            Value::Object(obj)
        }
        _ => Value::Null,
    }
}
