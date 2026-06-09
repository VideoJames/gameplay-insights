// All timestamp: i32 are placeholder for a real Timestamp type

use serde_json;
use serde::Serialize;
use serde::Deserialize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Payload {
	Event(Event),
	StateTransition(StateTransition),
	EntitySnapshot(EntitySnapshot)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Envelope {
	timestamp: i32,
	payload, Payload
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FieldValue {
	Text(String), 
	Int(i32), 
	Float(f32), 
	Bool(bool)
}

impl FieldValue {
	fn as_display(&self) -> String {
		match self {
			FieldValue::Text(value) => value.to_string(),
			FieldValue::Int(value) => format!("{value}"),
			FieldValue::Float(value) => format!("{value}"),
			FieldValue::Bool(value) => format!("{value}"),
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
	name: String,
	entity: Option<String>,
	fields: Vec<(String, FieldValue)>,
}

impl Event {
	pub fn new(name: &str) -> Self {
		Event { name: name.to_string(), fields: Vec::new(), entity: None }
	}
	
	pub fn field(mut self, key: &str, value: FieldValue) -> Self {
		self.fields.push((key.to_string(), value));
		self
	}
	
	fn summary(&self) -> String {
		let mut output = self.name.clone();
		output.push_str("\n");
		output.push_str(&format!("Timestamp: {}\n", self.timestamp));
		if let Some(x) = &self.entity {
			let entity_summary = format!("Entity: {}\n", x);
			output.push_str(&entity_summary);
		}
		for (key, value) in &self.fields {
			let pair = format!("{key}: {}\n", value.as_display());
			output.push_str(&pair);
		}
		output
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateTransition {
	previous_state: String,
	new_state: String,
	trigger: String,
	entity: String
}

impl StateTransition {
	pub fn new(previous_state: &str, new_state: &str, trigger: &str, entity: &str) -> Self {
		StateTransition { 
			previous_state: previous_state.to_string(),
			new_state: new_state.to_string(),
			trigger: trigger.to_string(),
			entity: entity.to_string()
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntitySnapshot {
	entity: String,
	fields: Vec<(String, FieldValue)>
}

impl EntitySnapshot {
	pub fn new(name: &str) -> Self {
		EntitySnapshot { entity: name.to_string(), fields: Vec::new() }
	}
	
	pub fn field(mut self, key: &str, value: FieldValue) -> Self {
		self.fields.push((key.to_string(), value));
		self
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn new_event_with_single_field() {
		let event = Event::new("Test Event")
			.field("damage", FieldValue::Int(10));
		let serialized_event = serde_json::to_string(&event);
		dbg!(serialized_event);
	}
}
