use std::{
	cell::RefCell,
	collections::HashMap,
	rc::{Rc, Weak},
};

use result_or_err::ResultOrErr;

struct GroupInternal<T> {
	subscribers: RefCell<HashMap<usize, Box<dyn FnMut(&T) -> ()>>>,
	last_id: RefCell<usize>,
}
impl<T> GroupInternal<T> {
	fn new() -> Self {
		Self { subscribers: RefCell::new(HashMap::new()), last_id: RefCell::new(0) }
	}
	fn get_next_id(&self) -> Result<usize, ()> {
		let subscribers = self.subscribers.try_borrow().or_err(())?;
		let mut last_id = self.last_id.try_borrow_mut().or_err(())?;
		let start = *last_id;
		loop {
			*last_id = last_id.wrapping_add(1);
			if !subscribers.contains_key(&last_id) {
				break;
			}
			if *last_id == start {
				return Err(());
			}
		}
		Ok(*last_id)
	}
	fn notify(&self, argument: T) -> Result<(), ()> {
		let mut subscribers = self.subscribers.try_borrow_mut().or_err(())?;
		for subscriber in subscribers.values_mut() {
			(*subscriber)(&argument);
		}
		Ok(())
	}
}

pub struct GroupToken<T> {
	group: Weak<GroupInternal<T>>,
	id: usize,
}
impl<T> GroupToken<T> {
	fn new(group: &Group<T>, id: usize) -> Self {
		Self { group: Rc::downgrade(&group.internal), id }
	}
	fn deregister_internal(&mut self) -> Result<(), ()> {
		let Some(group) = self.group.upgrade() else { return Ok(()) };
		let mut subscribers = group.subscribers.try_borrow_mut().or_err(())?;
		_ = subscribers.remove(&self.id).ok_or(())?;
		Ok(())
	}
	pub fn deregister(mut self) -> Result<(), ()> {
		self.deregister_internal()
	}
	pub fn notify(&self, argument: T) -> Result<(), ()> {
		self.group.upgrade().ok_or(())?.notify(argument)
	}
	pub fn forget(mut self) {
		self.group = Weak::new();
	}
}
impl<T> Drop for GroupToken<T> {
	fn drop(&mut self) {
		_ = self.deregister_internal();
	}
}

pub struct Group<TArg> {
	internal: Rc<GroupInternal<TArg>>,
}
impl<TArg> Group<TArg> {
	pub fn new() -> Self {
		Self { internal: Rc::new(GroupInternal::new()) }
	}
	pub fn register(&self, callback: impl FnMut(&TArg) + 'static) -> Result<GroupToken<TArg>, ()> {
		let id = self.internal.get_next_id()?;
		let mut subscribers = self.internal.subscribers.try_borrow_mut().or_err(())?;
		subscribers.insert(id, Box::new(callback));
		Ok(GroupToken::new(self, id))
	}
	pub fn notify(&self, argument: TArg) -> Result<(), ()> {
		self.internal.notify(argument)
	}
}
