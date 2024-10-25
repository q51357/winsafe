use std::cell::RefCell;
use std::mem::ManuallyDrop;
use std::rc::Rc;

use crate::co;
use crate::decl::*;
use crate::gui::*;
use crate::msg::*;
use crate::prelude::*;

/// A single item of a [`ListView`](crate::gui::ListView) control.
///
/// **Note:** Each object keeps the zero-based index of an item. If new items
/// are added/removed from the list view control, the object may then point to a
/// different item.
///
/// You cannot directly instantiate this object, it is created internally by the
/// control.
pub struct ListViewItem<'a, T: 'static = ()> {
	owner: &'a ListView<T>,
	index: u32,
}

impl<'a, T> Clone for ListViewItem<'a, T> { // https://stackoverflow.com/q/39415052/6923555
	fn clone(&self) -> Self {
		Self {
			owner: self.owner,
			index: self.index,
		}
	}
}
impl<'a, T> Copy for ListViewItem<'a, T> {}

impl<'a, T> ListViewItem<'a, T> {
	#[must_use]
	pub(in crate::gui) const fn new(owner: &'a ListView<T>, index: u32) -> Self {
		Self { owner, index }
	}

	/// Returns a [`Rc`](std::rc::Rc)/[`RefCell`](std::cell::RefCell) with the
	/// stored data by sending an [`lvm::GetItem`](crate::msg::lvm::GetItem)
	/// message.
	///
	/// Returns `None` if the `ListView` holds a `()`, or if the item holds an
	/// invalid index.
	#[must_use]
	pub fn data(&self) -> Option<Rc<RefCell<T>>> {
		self.data_lparam()
			.map(|pdata| {
				let rc_data = ManuallyDrop::new(unsafe { Rc::from_raw(pdata) });
				Rc::clone(&rc_data)
			})
	}

	#[must_use]
	pub(in crate::gui) fn data_lparam(&self) -> Option<*mut RefCell<T>> {
		let mut lvi = LVITEM::default();
		lvi.iItem = self.index as _;
		lvi.mask = co::LVIF::PARAM;

		unsafe {
			self.owner.hwnd()
				.SendMessage(lvm::GetItem { lvitem: &mut lvi })
		}.unwrap();

		match lvi.lParam {
			0 => None,
			lp => Some(lp as _),
		}
	}

	/// Deletes the item by sending an
	/// [`lvm::DeleteItem`](crate::msg::lvm::DeleteItem) message.
	pub fn delete(&self) {
		unsafe {
			self.owner.hwnd()
				.SendMessage(lvm::DeleteItem { index: self.index, })
		}.unwrap();
	}

	/// Scrolls the list by sending an
	/// [`lvm::EnsureVisible`](crate::msg::lvm::EnsureVisible) message so that
	/// the item is visible in the list.
	pub fn ensure_visible(&self) {
		unsafe {
			self.owner.hwnd()
				.SendMessage(lvm::EnsureVisible {
					index: self.index,
					entirely_visible: true,
				})
		}.unwrap();
	}

	/// Sets the item as the focused one sending an
	/// [`lvm:SetItemState`](crate::msg::lvm::SetItemState) message.
	pub fn focus(&self) {
		let mut lvi = LVITEM::default();
		lvi.stateMask = co::LVIS::FOCUSED;
		lvi.state = co::LVIS::FOCUSED;

		unsafe {
			self.owner.hwnd()
				.SendMessage(lvm::SetItemState {
					index: Some(self.index),
					lvitem: &lvi,
				})
		}.unwrap();
	}

	/// Retrieves the icon index of the item by sending an
	/// [`lvm::GetItem`](crate::msg::lvm::GetItem) message.
	#[must_use]
	pub fn icon_index(&self) -> Option<u32> {
		let mut lvi = LVITEM::default();
		lvi.iItem = self.index as _;
		lvi.mask = co::LVIF::IMAGE;

		unsafe {
			self.owner.hwnd()
				.SendMessage(lvm::SetItem { lvitem: &mut lvi })
		}.unwrap();

		match lvi.iImage {
			-1 => None,
			idx => Some(idx as _),
		}
	}

	/// Returns the zero-based index of the item.
	#[must_use]
	pub const fn index(&self) -> u32 {
		self.index
	}

	/// Tells if the item is the focused one by sending an
	/// [`lvm::GetItemState`](crate::msg::lvm::GetItemState) message.
	#[must_use]
	pub fn is_focused(&self) -> bool {
		unsafe {
			self.owner.hwnd()
				.SendMessage(lvm::GetItemState {
					index: self.index,
					mask: co::LVIS::FOCUSED,
				})
		}.has(co::LVIS::FOCUSED)
	}

	/// Tells if the item is selected by sending an
	/// [`lvm::GetItemState`](crate::msg::lvm::GetItemState) message.
	#[must_use]
	pub fn is_selected(&self) -> bool {
		unsafe {
			self.owner.hwnd()
				.SendMessage(lvm::GetItemState {
					index: self.index,
					mask: co::LVIS::SELECTED,
				})
		}.has(co::LVIS::SELECTED)
	}

	/// Tells if the item is currently visible by sending an
	/// [`lvm::IsItemVisible`](crate::msg::lvm::IsItemVisible) message.
	#[must_use]
	pub fn is_visible(&self) -> bool {
		unsafe {
			self.owner.hwnd()
				.SendMessage(lvm::IsItemVisible { index: self.index })
		}
	}

	/// Retrieves the unique ID for the item index by sending an
	/// [`lvm::MapIndexToId`](crate::msg::lvm::MapIndexToId) message.
	///
	/// If the item index has became invalid, returns `None`.
	#[must_use]
	pub fn map_index_to_id(&self) -> u32 {
		unsafe {
			self.owner.hwnd()
				.SendMessage(lvm::MapIndexToId { index: self.index })
		}.unwrap()
	}

	/// Retrieves the bound rectangle of item by sending an
	/// [`lvm::GetItemRect`](crate::msg::lvm::GetItemRect) message.
	#[must_use]
	pub fn rect(&self, portion: co::LVIR) -> RECT {
		let mut rc = RECT::default();
		unsafe {
			self.owner.hwnd()
				.SendMessage(lvm::GetItemRect {
					index: self.index,
					rect: &mut rc,
					portion,
				})
		}.unwrap();
		rc
	}

	/// Sets or removes the selection from the item by sending an
	/// [`lvm::SetItemState`](crate::msg::lvm::SetItemState) message.
	pub fn select(&self, set: bool) {
		let mut lvi = LVITEM::default();
		lvi.stateMask = co::LVIS::SELECTED;
		if set { lvi.state = co::LVIS::SELECTED; }

		unsafe {
			self.owner.hwnd()
				.SendMessage(lvm::SetItemState {
					index: Some(self.index),
					lvitem: &lvi,
				})
		}.unwrap();
	}

	/// Sets the icon index of the item by sending an
	/// [`lvm::SetItem`](crate::msg::lvm::SetItem) message.
	pub fn set_icon_index(&self, icon_index: Option<u32>) {
		let mut lvi = LVITEM::default();
		lvi.iItem = self.index as _;
		lvi.mask = co::LVIF::IMAGE;
		lvi.iImage = icon_index.map_or(-1, |idx| idx as _);

		unsafe {
			self.owner.hwnd()
				.SendMessage(lvm::SetItem { lvitem: &mut lvi })
		}.unwrap();
	}

	/// Sets the text of the item under a column by sending an
	/// [`lvm::SetItemText`](crate::msg::lvm::SetItemText) message.
	pub fn set_text(&self, column_index: u32, text: &str) {
		let mut lvi = LVITEM::default();
		lvi.iSubItem = column_index as _;

		let mut wtext = WString::from_str(text);
		lvi.set_pszText(Some(&mut wtext));

		unsafe {
			self.owner.hwnd()
				.SendMessage(lvm::SetItemText {
					index: self.index,
					lvitem: &lvi,
				})
		}.unwrap();
	}

	/// Retrieves the text of an item under a column by sending an
	/// [`lvm::GetItemText`](crate::msg::lvm::GetItemText) message.
	#[must_use]
	pub fn text(&self, column_index: u32) -> String {
		// https://forums.codeguru.com/showthread.php?351972-Getting-listView-item-text-length
		let mut buf_sz = WString::SSO_LEN; // start with no string heap allocation
		loop {
			let mut lvi = LVITEM::default();
			lvi.iSubItem = column_index as _;

			let mut buf = WString::new_alloc_buf(buf_sz);
			lvi.set_pszText(Some(&mut buf));

			let returned_chars = unsafe {
				self.owner.hwnd() // char count without terminating null
					.SendMessage(lvm::GetItemText {
						index: self.index,
						lvitem: &mut lvi,
					})
			} + 1; // plus terminating null count

			if (returned_chars as usize) < buf_sz { // to break, must have at least 1 char gap
				return buf.to_string();
			}

			buf_sz *= 2; // double the buffer size to try again
		}
	}
}
