//! Assorted Win32 structs used by common controls.

#![allow(non_snake_case)]

use std::marker::PhantomData;

use crate::co;
use crate::handles::{HDC, HIMAGELIST, HTREEITEM};
use crate::privs::{L_MAX_URL_LENGTH, MAX_LINKID_TEXT};
use crate::structs::{COLORREF, NMHDR, POINT, RECT, SIZE, SYSTEMTIME};
use crate::WString;

/// [`BUTTON_IMAGELIST`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-button_imagelist)
/// struct.
#[repr(C)]
#[derive(Clone, Eq, PartialEq)]
pub struct BUTTON_IMAGELIST {
	pub himl: HIMAGELIST,
	pub margin: RECT,
	pub uAlign: co::BIA,
}

impl_default_zero!(BUTTON_IMAGELIST);

/// [`BUTTON_SPLITINFO`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-button_splitinfo)
/// struct.
#[repr(C)]
#[derive(Clone, Eq, PartialEq)]
pub struct BUTTON_SPLITINFO {
	pub mask: co::BCSIF,
	pub himlGlyph: HIMAGELIST,
	pub uSplitStyle: co::BCSS,
	pub size: SIZE,
}

impl_default_zero!(BUTTON_SPLITINFO);

/// [`IMAGELISTDRAWPARAMS`](https://docs.microsoft.com/en-us/windows/win32/api/commoncontrols/ns-commoncontrols-imagelistdrawparams)
/// struct.
#[repr(C)]
#[derive(Clone, Eq, PartialEq)]
pub struct IMAGELISTDRAWPARAMS {
	cbSize: u32,
	pub himl: HIMAGELIST,
	pub i: i32,
	pub hdcDst: HDC,
	pub x: i32,
	pub y: i32,
	pub cx: i32,
	pub cy: i32,
	pub xBitmap: i32,
	pub yBitmap: i32,
	pub rgbBk: COLORREF,
	pub rgbFg: COLORREF,
	pub fStyle: co::ILD,
	pub dwRop: co::ROP,
	pub fState: co::ILS,
	pub Frame: u32,
	pub crEffect: COLORREF,
}

impl Default for IMAGELISTDRAWPARAMS {
	fn default() -> Self {
		let mut obj = unsafe { std::mem::zeroed::<Self>() };
		obj.cbSize = std::mem::size_of::<Self>() as u32;
		obj
	}
}

/// [`LITEM`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-litem)
/// struct.
#[repr(C)]
#[derive(Clone, Eq, PartialEq)]
pub struct LITEM {
	pub mask: co::LIF,
	pub iLink: i32,
	pub state: co::LIS,
	pub stateMask: co::LIS,
	szID: [u16; MAX_LINKID_TEXT],
	szUrl: [u16; L_MAX_URL_LENGTH],
}

impl_default_zero!(LITEM);

impl LITEM {
	/// Returns the `szID` field.
	pub fn szID(&self) -> String {
		WString::from_wchars_slice(&self.szID).to_string()
	}

	/// Sets the `szID` field.
	pub fn get_szID(&mut self, text: &str) {
		WString::from_str(text).copy_to_slice(&mut self.szID);
	}

	/// Returns the `szUrl` field.
	pub fn szUrl(&self) -> String {
		WString::from_wchars_slice(&self.szUrl).to_string()
	}

	/// Sets the `szUrl` field.
	pub fn set_szUrl(&mut self, text: &str) {
		WString::from_str(text).copy_to_slice(&mut self.szUrl);
	}
}

/// [`LVCOLUMN`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-lvcolumnw)
/// struct.
#[repr(C)]
#[derive(Clone, Eq, PartialEq)]
pub struct LVCOLUMN<'a> {
	pub mask: co::LVCF,
	pub fmt: co::LVCFMT_C,
	pub cx: i32,
	pszText: *mut u16,
	cchTextMax: i32,
	pub iSubItem: i32,
	pub iImage: i32,
	pub iOrder: i32,
	pub cxMin: i32,
	pub cxDefault: i32,
	pub cxIdeal: i32,
	m_pszText: PhantomData<&'a u16>,
}

impl_default_zero!(LVCOLUMN, 'a);

impl<'a> LVCOLUMN<'a> {
	/// Sets the `pszText` and `cchTextMax` fields. The buffer will be resized to
	/// hold at least 64 chars.
	pub fn set_pszText(&mut self, buf: &'a mut WString) {
		if buf.buffer_size() < 64 { buf.realloc_buffer(64); } // arbitrary
		self.pszText = unsafe { buf.as_mut_ptr() };
		self.cchTextMax = buf.buffer_size() as i32;
	}
}

/// [`LVFINDINFO`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-lvfindinfow)
/// struct.
#[repr(C)]
#[derive(Clone, Eq, PartialEq)]
pub struct LVFINDINFO<'a> {
	pub flags: co::LVFI,
	psz: *const u16,
	pub lParam: isize,
	pub pt: POINT,
	pub vkDirection: co::VK_DIR,
	m_psz: PhantomData<&'a u16>,
}

impl_default_zero!(LVFINDINFO, 'a);

impl<'a> LVFINDINFO<'a> {
	/// Returns the `psz` field.
	pub fn psz(&self) -> String {
		WString::from_wchars_nullt(self.psz).to_string()
	}

	/// Sets the `psz` field.
	pub fn set_psz(&mut self, buf: &'a WString) {
		self.psz = unsafe { buf.as_ptr() };
	}
}

/// [`LVITEM`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-lvitemw)
/// struct.
#[repr(C)]
#[derive(Clone, Eq, PartialEq)]
pub struct LVITEM<'a> {
	pub mask: co::LVIF,
	pub iItem: i32,
	pub iSubItem: i32,
	pub state: co::LVIS,
	pub stateMask: co::LVIS,
	pszText: *mut u16,
	cchTextMax: i32,
	pub iImage: i32,
	pub lParam: isize,
	pub iIndent: i32,
	pub iGroupId: co::LVI_GROUPID,
	pub cColumns: u32,
	pub puColumns: *mut i32,
	pub piColFmt: *mut co::LVCFMT_I,
	pub iGroup: i32,
	m_pszText: PhantomData<&'a u16>,
}

impl_default_zero!(LVITEM, 'a);

impl<'a> LVITEM<'a> {
	/// Sets the `pszText` and `cchTextMax` fields. The buffer will be resized to
	/// hold at least 64 chars.
	pub fn set_pszText(&mut self, buf: &'a mut WString) {
		if buf.buffer_size() < 64 { buf.realloc_buffer(64); } // arbitrary
		self.pszText = unsafe { buf.as_mut_ptr() };
		self.cchTextMax = buf.buffer_size() as i32;
	}
}

/// [`NMBCDROPDOWN`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmbcdropdown)
/// struct.
#[repr(C)]
#[derive(Default, Clone, Eq, PartialEq)]
pub struct NMBCDROPDOWN {
	pub hdr: NMHDR,
	pub rcButton: RECT,
}

/// [`NMBCHOTITEM`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmbchotitem)
/// struct.
#[repr(C)]
#[derive(Default, Clone, Eq, PartialEq)]
pub struct NMBCHOTITEM {
	pub hdr: NMHDR,
	pub dwFlags: co::HICF,
}

/// [`NMCHAR`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmchar)
/// struct.
#[repr(C)]
#[derive(Default, Clone, Eq, PartialEq)]
pub struct NMCHAR {
	pub hdr: NMHDR,
	pub ch: u32,
	pub dwItemPrev: u32,
	pub dwItemNext: u32,
}

/// [`NMCUSTOMDRAW`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmcustomdraw)
/// struct.
#[repr(C)]
#[derive(Clone, Eq, PartialEq)]
pub struct NMCUSTOMDRAW {
	pub hdr: NMHDR,
	pub dwDrawStage: co::CDDS,
	pub hdc: HDC,
	pub rc: RECT,
	pub dwItemSpec: usize,
	pub uItemState: co::CDIS,
	pub lItemlParam: isize,
}

impl_default_zero!(NMCUSTOMDRAW);

/// [`NMDATETIMECHANGE`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmdatetimechange)
/// struct
#[repr(C)]
#[derive(Default, Clone, Eq, PartialEq)]
pub struct NMDATETIMECHANGE {
	pub nmhdr: NMHDR,
	pub dwFlags: co::GDT,
	pub st: SYSTEMTIME,
}

/// [`NMDATETIMEFORMAT`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmdatetimeformatw)
/// struct.
#[repr(C)]
#[derive(Clone, Eq, PartialEq)]
pub struct NMDATETIMEFORMAT<'a> {
	pub nmhdr: NMHDR,
	pszFormat: *const u16,
	pub st: SYSTEMTIME,
	pszDisplay: *const u16,
	szDisplay: [u16; 64], // used as a buffer to pszDisplay
	m_pszFormat: PhantomData<&'a u16>,
}

impl_default_zero!(NMDATETIMEFORMAT, 'a);

impl<'a> NMDATETIMEFORMAT<'a> {
	/// Returns the `pszFormat` field.
	pub fn pszFormat(&self) -> String {
		WString::from_wchars_nullt(self.pszFormat).to_string()
	}

	/// Sets the `pszFormat` field.
	pub fn set_pszFormat(&mut self, buf: &'a WString) {
		self.pszFormat = unsafe { buf.as_ptr() };
	}

	/// Returns the `pszDisplay` field.
	pub fn pszDisplay(&self) -> String {
		WString::from_wchars_nullt(self.pszDisplay).to_string()
	}

	/// Sets the `pszDisplay` field.
	pub fn set_pszDisplay(&mut self, text: &str) {
		WString::from_str(text).copy_to_slice(&mut self.szDisplay);
	}
}

/// [`NMDATETIMEFORMATQUERY`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmdatetimeformatqueryw)
/// struct.
#[repr(C)]
#[derive(Clone, Eq, PartialEq)]
pub struct NMDATETIMEFORMATQUERY<'a> {
	pub nmhdr: NMHDR,
	pszFormat: *const u16,
	pub szMax: SIZE,
	m_pszFormat: PhantomData<&'a u16>,
}

impl_default_zero!(NMDATETIMEFORMATQUERY, 'a);

impl<'a> NMDATETIMEFORMATQUERY<'a> {
	/// Returns the `pszFormat` field.
	pub fn pszFormat(&self) -> String {
		WString::from_wchars_nullt(self.pszFormat).to_string()
	}

	/// Sets the `pszFormat` field.
	pub fn set_pszFormat(&mut self, buf: &'a WString) {
		self.pszFormat = unsafe { buf.as_ptr() };
	}
}

/// [`NMDATETIMESTRING`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmdatetimestringw)
/// struct.
#[repr(C)]
#[derive(Clone, Eq, PartialEq)]
pub struct NMDATETIMESTRING<'a> {
	pub nmhdr: NMHDR,
	pszUserString: *const u16,
	pub st: SYSTEMTIME,
	pub dwFlags: co::GDT,
	m_pszUserString: PhantomData<&'a u16>,
}

impl_default_zero!(NMDATETIMESTRING, 'a);

impl<'a> NMDATETIMESTRING<'a> {
	/// Returns the `pszUserString` field.
	pub fn pszUserString(&self) -> String {
		WString::from_wchars_nullt(self.pszUserString).to_string()
	}

	/// Sets the `pszUserString` field.
	pub fn set_pszUserString(&mut self, buf: &'a WString) {
		self.pszUserString = unsafe { buf.as_ptr() };
	}
}

/// [`NMDATETIMEWMKEYDOWN`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmdatetimewmkeydownw)
/// struct.
pub struct NMDATETIMEWMKEYDOWN<'a> {
	pub nmhdr: NMHDR,
	pub nVirtKey: i32,
	pszFormat: *const u16,
	pub st: SYSTEMTIME,
	m_pszFormat: PhantomData<&'a u16>,
}

impl_default_zero!(NMDATETIMEWMKEYDOWN, 'a);

impl<'a> NMDATETIMEWMKEYDOWN<'a> {
	/// Returns the `pszFormat` field.
	pub fn pszFormat(&self) -> String {
		WString::from_wchars_nullt(self.pszFormat).to_string()
	}

	/// Sets the `pszFormat` field.
	pub fn set_pszFormat(&mut self, buf: &'a WString) {
		self.pszFormat = unsafe { buf.as_ptr() };
	}
}

/// [`NMITEMACTIVATE`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmitemactivate)
/// struct.
#[repr(C)]
#[derive(Default, Clone, Eq, PartialEq)]
pub struct NMITEMACTIVATE {
	pub hdr: NMHDR,
	pub iItem: i32,
	pub iSubItem: i32,
	pub uNewState: co::LVIS,
	pub uOldState: co::LVIS,
	pub uChanged: co::LVIF,
	pub ptAction: POINT,
	pub lParam: isize,
	pub uKeyFlags: co::LVKF,
}

/// [`NMIPADDRESS`](https://docs.microsoft.com/en-us/windows/win32/api/Commctrl/ns-commctrl-nmipaddress)
/// struct.
#[repr(C)]
#[derive(Default, Clone, Eq, PartialEq)]
pub struct NMIPADDRESS {
	pub hdr: NMHDR,
	pub iField: i32,
	pub iValue: i32,
}

/// [`NMLINK`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmlink)
/// struct.
#[repr(C)]
#[derive(Default, Clone, Eq, PartialEq)]
pub struct NMLINK {
	pub hdr: NMHDR,
	pub item: LITEM,
}

/// [`NMLISTVIEW`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmlistview)
/// struct.
#[repr(C)]
#[derive(Default, Clone, Eq, PartialEq)]
pub struct NMLISTVIEW {
	pub hdr: NMHDR,
	pub iItem: i32,
	pub iSubItem: i32,
	pub uNewState: co::LVIS,
	pub uOldState: co::LVIS,
	pub uChanged: co::LVIF,
	pub ptAction: POINT,
	pub lParam: isize,
}

/// [`NMLVCACHEHINT`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmlvcachehint)
/// struct
#[repr(C)]
#[derive(Default, Clone, Eq, PartialEq)]
pub struct NMLVCACHEHINT {
	pub hdr: NMHDR,
	pub iFrom: i32,
	pub iTo: i32,
}

/// [`NMLVDISPINFO`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmlvdispinfow)
/// struct.
#[repr(C)]
#[derive(Default, Clone, Eq, PartialEq)]
pub struct NMLVDISPINFO<'a> {
	pub hdr: NMHDR,
	pub item: LVITEM<'a>,
}

/// [`NMLVEMPTYMARKUP`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmlvemptymarkup)
/// struct.
#[repr(C)]
#[derive(Clone, Eq, PartialEq)]
pub struct NMLVEMPTYMARKUP {
	pub hdr: NMHDR,
	pub dwFlags: co::EMF,
	szMarkup: [u16; L_MAX_URL_LENGTH],
}

impl_default_zero!(NMLVEMPTYMARKUP);

impl NMLVEMPTYMARKUP {
	/// Returns the `szMarkup` field.
	pub fn szMarkup(&self) -> String {
		WString::from_wchars_slice(&self.szMarkup).to_string()
	}

	/// Sets the `szMarkup` field.
	pub fn get_szID(&mut self, text: &str) {
		WString::from_str(text).copy_to_slice(&mut self.szMarkup);
	}
}

/// [`NMLVFINDITEM`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmlvfinditemw)
/// struct.
#[repr(C)]
#[derive(Clone, Eq, PartialEq)]
pub struct NMLVFINDITEM<'a> {
	pub hdr: NMHDR,
	pub iStart: i32,
	pub lvfi: LVFINDINFO<'a>,
}

/// [`NMLVGETINFOTIP`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmlvgetinfotipw)
/// struct.
#[derive(Clone, Eq, PartialEq)]
pub struct NMLVGETINFOTIP<'a> {
	pub hdr: NMHDR,
	pub dwFlags: co::LVGIT,
	pszText: *mut u16,
	cchTextMax: i32,
	pub iItem: i32,
	pub iSubItem: i32,
	pub lParam: isize,
	m_pszText: PhantomData<&'a u16>,
}

impl_default_zero!(NMLVGETINFOTIP, 'a);

impl<'a> NMLVGETINFOTIP<'a> {
	/// Returns the `pszText` field.
	pub fn pszText(&self) -> String {
		WString::from_wchars_nullt(self.pszText).to_string()
	}

	/// Sets the `pszText` and `cchTextMax` fields. The buffer will be resized to
	/// hold at least 64 chars.
	pub fn set_pszText(&mut self, buf: &'a mut WString) {
		if buf.buffer_size() < 64 { buf.realloc_buffer(64); } // arbitrary
		self.pszText = unsafe { buf.as_mut_ptr() };
		self.cchTextMax = buf.buffer_size() as i32;
	}
}

/// [`NMLVKEYDOWN`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmlvkeydown)
/// struct.
#[repr(C)]
#[derive(Default, Clone, Eq, PartialEq)]
pub struct NMLVKEYDOWN {
	pub hdr: NMHDR,
	pub wVKey: co::VK,
	flags: u32,
}

/// [`NMLVLINK`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmlvlink)
/// struct.
#[repr(C)]
#[derive(Default, Clone, Eq, PartialEq)]
pub struct NMLVLINK {
	pub hdr: NMHDR,
	pub link: LITEM,
	pub iItem: i32,
	pub iSubItem: i32,
}

/// [`NMLVODSTATECHANGE`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmlvodstatechange)
/// struct.
#[repr(C)]
#[derive(Default, Clone, Eq, PartialEq)]
pub struct NMLVODSTATECHANGE {
	pub hdr: NMHDR,
	pub iFrom: i32,
	pub iTo: i32,
	pub uNewState: co::LVIS,
	pub uOldState: co::LVIS,
}

/// [`NMLVSCROLL`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmlvscroll)
/// struct.
#[repr(C)]
#[derive(Default, Clone, Eq, PartialEq)]
pub struct NMLVSCROLL {
	pub hdr: NMHDR,
	pub dx: i32,
	pub dy: i32,
}

/// [`NMMOUSE`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmmouse)
/// struct.
#[repr(C)]
#[derive(Default, Clone, Eq, PartialEq)]
pub struct NMMOUSE {
	pub hdr: NMHDR,
	pub dwItemSpec: usize,
	pub dwItemData: usize,
	pub pt: POINT,
	pub dwHitInfo: isize,
}

/// [`NMTVASYNCDRAW`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmtvasyncdraw)
/// method.
#[repr(C)]
#[derive(Clone, Eq, PartialEq)]
pub struct NMTVASYNCDRAW<'a> {
	pub hdr: NMHDR,
	pimldp: *const IMAGELISTDRAWPARAMS,
	pub hr: co::ERROR,
	pub hItem: HTREEITEM,
	pub lParam: isize,
	pub dwRetFlags: co::ADRF,
	pub iRetImageIndex: i32,
	m_pimldp: PhantomData<&'a IMAGELISTDRAWPARAMS>,
}

impl_default_zero!(NMTVASYNCDRAW, 'a);

impl<'a> NMTVASYNCDRAW<'a> {
	/// Returns the `pimldp` field.
	pub fn pimldp(&self) -> &IMAGELISTDRAWPARAMS {
		unsafe { &*self.pimldp }
	}
}
