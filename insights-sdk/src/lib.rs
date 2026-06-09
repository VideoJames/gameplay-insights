// All timestamp: i32 are placeholder for a real Timestamp type

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct Event {
	name: String,
	timestamp: i32,
	entity: Option<String>,
	fields: Vec<(String, FieldValue)>,
}

impl Event {
	pub fn new(name: &str) -> Self {
		Event { name: name.to_string(), timestamp: 0, fields: Vec::new(), entity: None }
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

}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn new_event_with_single_field() {
		let event = Event::new("Test Event")
			.field("damage", FieldValue::Int(10));
		dbg!(event);
	}
}
