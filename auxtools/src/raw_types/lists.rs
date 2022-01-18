use super::values;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ListId(pub u32);

#[repr(u8)]
#[allow(unused)]
#[derive(Debug)]
pub enum Color {
	Red = 0,
	Black = 1,
}

#[repr(C)]
#[derive(Debug)]
pub struct AssociativeListEntry {
	pub key: values::Value,
	pub value: values::Value,
	pub right: *mut AssociativeListEntry,
	pub left: *mut AssociativeListEntry,
	pub parent: *mut AssociativeListEntry,
	pub color: Color,
	pub padding:[u8;3]
}

#[repr(C)]
#[derive(Debug)]
pub struct List {
	pub vector_part: *mut values::Value,
	pub assoc_part: *mut AssociativeListEntry,
	pub allocated: u32,
	pub length: u32,
	pub refcount: u32,
	pub unknown: *mut ValueArrayData,
}

#[repr(C)]
#[derive(Debug)]
pub struct ValueArrayData {
	pub value_pointer: *mut values::Value,
	pub unkn1: u32,
	pub size: usize,
	pub unkn2: u32,
}
