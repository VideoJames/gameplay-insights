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
			FieldValue::Int(value) => !"{value}",
			FieldValue::Float(value) => "{value}".to_string(),
			FieldValue::Bool(value) => "{value}".to_string(),			
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
	pub fn new(name: &str) -> Event
	{
		Event { name: name.to_string(), timestamp: 0, fields: Vec::new(), entity: None }
	}
	
	pub fn field(mut self, key: &str, value: FieldValue) -> Self
	{
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
		dbg!(event);
	}
}
