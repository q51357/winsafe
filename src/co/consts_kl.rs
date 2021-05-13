#![allow(non_snake_case)]

use crate::co::{ACCESS_RIGHTS, STANDARD_RIGHTS, WM, WS};

const_type! { KEY, u32,
	/// [`RegOpenKeyEx`](crate::HKEY::RegOpenKeyEx) `samDesired` (`u32`).
	=>
	QUERY_VALUE, 0x0001
	SET_VALUE, 0x0002
	CREATE_SUB_KEY, 0x0004
	ENUMERATE_SUB_KEYS, 0x0008
	NOTIFY, 0x0010
	CREATE_LINK, 0x0020
	WOW64_32KEY, 0x0200
	WOW64_64KEY, 0x0100
	WOW64_RES, 0x0300
	READ, (STANDARD_RIGHTS::READ.0 | Self::QUERY_VALUE.0 | Self::ENUMERATE_SUB_KEYS.0 | Self::NOTIFY.0) & !ACCESS_RIGHTS::SYNCHRONIZE.0
	WRITE, (STANDARD_RIGHTS::WRITE.0 | Self::SET_VALUE.0 | Self::CREATE_SUB_KEY.0) & !ACCESS_RIGHTS::SYNCHRONIZE.0
	EXECUTE, Self::READ.0 & !ACCESS_RIGHTS::SYNCHRONIZE.0
	ALL_ACCESS, (STANDARD_RIGHTS::ALL.0 | Self::QUERY_VALUE.0 | Self::SET_VALUE.0 | Self::CREATE_SUB_KEY.0 | Self::ENUMERATE_SUB_KEYS.0 | Self::NOTIFY.0 | Self::CREATE_LINK.0) & !ACCESS_RIGHTS::SYNCHRONIZE.0
}

const_type! { LANG, u16,
	/// Language
	/// [identifier](https://docs.microsoft.com/en-us/windows/win32/intl/language-identifier-constants-and-strings)
	/// (`u16`).
	=>
	NEUTRAL, 0x00
	INVARIANT, 0x7f
	AFRIKAANS, 0x36
	ALBANIAN, 0x1c
	ALSATIAN, 0x84
	AMHARIC, 0x5e
	ARABIC, 0x01
	ARMENIAN, 0x2b
	ASSAMESE, 0x4d
	AZERI, 0x2c
	AZERBAIJANI, 0x2c
	BANGLA, 0x45
	BASHKIR, 0x6d
	BASQUE, 0x2d
	BELARUSIAN, 0x23
	BENGALI, 0x45
	BRETON, 0x7e
	BOSNIAN, 0x1a
	BOSNIAN_NEUTRAL, 0x781a
	BULGARIAN, 0x02
	CATALAN, 0x03
	CENTRAL_KURDISH, 0x92
	CHEROKEE, 0x5c
	CHINESE, 0x04
	CHINESE_SIMPLIFIED, 0x04
	CHINESE_TRADITIONAL, 0x7c04
	CORSICAN, 0x83
	CROATIAN, 0x1a
	CZECH, 0x05
	DANISH, 0x06
	DARI, 0x8c
	DIVEHI, 0x65
	DUTCH, 0x13
	ENGLISH, 0x09
	ESTONIAN, 0x25
	FAEROESE, 0x38
	FARSI, 0x29
	FILIPINO, 0x64
	FINNISH, 0x0b
	FRENCH, 0x0c
	FRISIAN, 0x62
	FULAH, 0x67
	GALICIAN, 0x56
	GEORGIAN, 0x37
	GERMAN, 0x07
	GREEK, 0x08
	GREENLANDIC, 0x6f
	GUJARATI, 0x47
	HAUSA, 0x68
	HAWAIIAN, 0x75
	HEBREW, 0x0d
	HINDI, 0x39
	HUNGARIAN, 0x0e
	ICELANDIC, 0x0f
	IGBO, 0x70
	INDONESIAN, 0x21
	INUKTITUT, 0x5d
	IRISH, 0x3c
	ITALIAN, 0x10
	JAPANESE, 0x11
	KANNADA, 0x4b
	KASHMIRI, 0x60
	KAZAK, 0x3f
	KHMER, 0x53
	KICHE, 0x86
	KINYARWANDA, 0x87
	KONKANI, 0x57
	KOREAN, 0x12
	KYRGYZ, 0x40
	LAO, 0x54
	LATVIAN, 0x26
	LITHUANIAN, 0x27
	LOWER_SORBIAN, 0x2e
	LUXEMBOURGISH, 0x6e
	MACEDONIAN, 0x2f
	MALAY, 0x3e
	MALAYALAM, 0x4c
	MALTESE, 0x3a
	MANIPURI, 0x58
	MAORI, 0x81
	MAPUDUNGUN, 0x7a
	MARATHI, 0x4e
	MOHAWK, 0x7c
	MONGOLIAN, 0x50
	NEPALI, 0x61
	NORWEGIAN, 0x14
	OCCITAN, 0x82
	ODIA, 0x48
	ORIYA, 0x48
	PASHTO, 0x63
	PERSIAN, 0x29
	POLISH, 0x15
	PORTUGUESE, 0x16
	PULAR, 0x67
	PUNJABI, 0x46
	QUECHUA, 0x6b
	ROMANIAN, 0x18
	ROMANSH, 0x17
	RUSSIAN, 0x19
	SAKHA, 0x85
	SAMI, 0x3b
	SANSKRIT, 0x4f
	SCOTTISH_GAELIC, 0x91
	SERBIAN, 0x1a
	SERBIAN_NEUTRAL, 0x7c1a
	SINDHI, 0x59
	SINHALESE, 0x5b
	SLOVAK, 0x1b
	SLOVENIAN, 0x24
	SOTHO, 0x6c
	SPANISH, 0x0a
	SWAHILI, 0x41
	SWEDISH, 0x1d
	SYRIAC, 0x5a
	TAJIK, 0x28
	TAMAZIGHT, 0x5f
	TAMIL, 0x49
	TATAR, 0x44
	TELUGU, 0x4a
	THAI, 0x1e
	TIBETAN, 0x51
	TIGRIGNA, 0x73
	TIGRINYA, 0x73
	TSWANA, 0x32
	TURKISH, 0x1f
	TURKMEN, 0x42
	UIGHUR, 0x80
	UKRAINIAN, 0x22
	UPPER_SORBIAN, 0x2e
	URDU, 0x20
	UZBEK, 0x43
	VALENCIAN, 0x03
	VIETNAMESE, 0x2a
	WELSH, 0x52
	WOLOF, 0x88
	XHOSA, 0x34
	YAKUT, 0x85
	YI, 0x78
	YORUBA, 0x6a
	ZULU, 0x35
}

const_type_wm! { LB,
	/// List box control
	/// [messages](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-list-box-control-reference-messages)
	/// (`u32`), convertible to [`WM`](crate::co::WM).
	=>
	=>
	ADDSTRING, 0x0180
	INSERTSTRING, 0x0181
	DELETESTRING, 0x0182
	SELITEMRANGEEX, 0x0183
	RESETCONTENT, 0x0184
	SETSEL, 0x0185
	SETCURSEL, 0x0186
	GETSEL, 0x0187
	GETCURSEL, 0x0188
	GETTEXT, 0x0189
	GETTEXTLEN, 0x018a
	GETCOUNT, 0x018b
	SELECTSTRING, 0x018c
	DIR, 0x018d
	GETTOPINDEX, 0x018e
	FINDSTRING, 0x018f
	GETSELCOUNT, 0x0190
	GETSELITEMS, 0x0191
	SETTABSTOPS, 0x0192
	GETHORIZONTALEXTENT, 0x0193
	SETHORIZONTALEXTENT, 0x0194
	SETCOLUMNWIDTH, 0x0195
	ADDFILE, 0x0196
	SETTOPINDEX, 0x0197
	GETITEMRECT, 0x0198
	GETITEMDATA, 0x0199
	SETITEMDATA, 0x019a
	SELITEMRANGE, 0x019b
	SETANCHORINDEX, 0x019c
	GETANCHORINDEX, 0x019d
	SETCARETINDEX, 0x019e
	GETCARETINDEX, 0x019f
	SETITEMHEIGHT, 0x01a0
	GETITEMHEIGHT, 0x01a1
	FINDSTRINGEXACT, 0x01a2
	SETLOCALE, 0x01a5
	GETLOCALE, 0x01a6
	SETCOUNT, 0x01a7
	INITSTORAGE, 0x01a8
	ITEMFROMPOINT, 0x01a9
	GETLISTBOXINFO, 0x01b2
}

const_type_cmd! { LBN,
	/// List box control `WM_COMMAND`
	/// [notifications](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-list-box-control-reference-notifications)
	/// (`u16`), convertible to [`CMD`](crate::co::CMD).
	=>
	ERRSPACE, (0 -2) as u16
	SELCHANGE, 1
	DBLCLK, 2
	SELCANCEL, 3
	SETFOCUS, 4
	KILLFOCUS, 5
}

const_type_ws! { LBS,
	/// List box control
	/// [styles](https://docs.microsoft.com/en-us/windows/win32/controls/list-box-styles)
	/// (`u32`), convertible to [`WS`](crate::co::WS).
	=>
	NOTIFY, 0x0001
	SORT, 0x0002
	NOREDRAW, 0x0004
	MULTIPLESEL, 0x0008
	OWNERDRAWFIXED, 0x0010
	OWNERDRAWVARIABLE, 0x0020
	HASSTRINGS, 0x0040
	USETABSTOPS, 0x0080
	NOINTEGRALHEIGHT, 0x0100
	MULTICOLUMN, 0x0200
	WANTKEYBOARDINPUT, 0x0400
	EXTENDEDSEL, 0x0800
	DISABLENOSCROLL, 0x1000
	NODATA, 0x2000
	NOSEL, 0x4000
	COMBOBOX, 0x8000
	STANDARD, Self::NOTIFY.0 | Self::SORT.0 | WS::VSCROLL.0 | WS::BORDER.0
}

const_type! { LIF, u32,
	/// [`LITEM`](crate::LITEM) `mask` (`u32`).
	=>
	ITEMINDEX, 0x0000_0001
	STATE, 0x0000_0002
	ITEMID, 0x0000_0004
	URL, 0x0000_0008
}

const_type! { LIS, u32,
	/// [`LITEM`](crate::LITEM) `state` (`u32`).
	=>
	FOCUSED, 0x0000_0001
	ENABLED, 0x0000_0002
	VISITED, 0x0000_0004
	HOTTRACK, 0x0000_0008
	DEFAULTCOLORS, 0x0000_0010
}

const_type! { LLMHF, u32,
	/// [`MSLLHOOKSTRUCT`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-msllhookstruct)
	/// `flags` (`u32`).
	=>
	INJECTED, 0x0000_0001
	LOWER_IL_INJECTED, 0x0000_0002
}

const_type_wm! { LM,
	/// SysLink control
	/// [messages](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-syslink-control-reference-messages)
	/// (`u32`), convertible to [`WM`](crate::co::WM).
	=>
	=>
	HITTEST, WM::USER.0 + 0x300
	GETIDEALHEIGHT, WM::USER.0 + 0x301
	SETITEM, WM::USER.0 + 0x302
	GETITEM, WM::USER.0 + 0x303
	GETIDEALSIZE, Self::GETIDEALHEIGHT.0
}

const_type! { LR, u32,
	/// [`LoadImageBitmap`](crate::HINSTANCE::LoadImageBitmap),
	/// [`LoadImageCursor`](crate::HINSTANCE::LoadImageCursor) and
	/// [`LoadImageIcon`](crate::HINSTANCE::LoadImageIcon) `fuLoad`.
	=>
	DEFAULTCOLOR, 0x0000_0000
	MONOCHROME, 0x0000_0001
	COLOR, 0x0000_0002
	COPYRETURNORG, 0x0000_0004
	COPYDELETEORG, 0x0000_0008
	LOADFROMFILE, 0x0000_0010
	LOADTRANSPARENT, 0x0000_0020
	DEFAULTSIZE, 0x0000_0040
	VGACOLOR, 0x0000_0080
	LOADMAP3DCOLORS, 0x0000_1000
	CREATEDIBSECTION, 0x0000_2000
	COPYFROMRESOURCE, 0x0000_4000
	SHARED, 0x0000_8000
}

const_type! { LSFW, u32,
	/// [`LockSetForegroundWindow`](crate::LockSetForegroundWindow) `uLockCode`
	/// (`u32`).
	=>
	LOCK, 1
	UNLOCK, 2
}

const_type! { LV_VIEW, u32,
	/// ListView
	/// [views](https://docs.microsoft.com/en-us/windows/win32/controls/list-view-controls-overview)
	/// (`u32`).
	=>
	ICON, 0x0000
	DETAILS, 0x0001
	SMALLICON, 0x0002
	LIST, 0x0003
	TILE, 0x0004
}

const_type! { LVA, u16,
	/// [`LVM_ARRANGE`](crate::msg::lvm::Arrange) arrangement (`u16`).
	=>
	DEFAULT, 0x0000
	SNAPTOGRID, 0x0005
}

const_type! { LVCF, u32,
	/// [`LVCOLUMN`](crate::LVCOLUMN) `mask` (`u32`).
	=>
	DEFAULTWIDTH, 0x0080
	FMT, 0x0001
	IDEALWIDTH, 0x0100
	IMAGE, 0x0010
	MINWIDTH, 0x0040
	ORDER, 0x0020
	SUBITEM, 0x0008
	TEXT, 0x0004
	WIDTH, 0x0002
}

const_type! { LVCFMT_C, i32,
	/// [`LVCOLUMN`](crate::LVCOLUMN) `mask` (`i32`).
	=>
	LEFT, 0x0000
	RIGHT, 0x0001
	CENTER, 0x0002
	JUSTIFYMASK, 0x0003
	IMAGE, 0x0800
	BITMAP_ON_RIGHT, 0x1000
	COL_HAS_IMAGES, 0x8000
	FIXED_WIDTH, 0x00100
	NO_DPI_SCALE, 0x40000
	FIXED_RATIO, 0x80000
	SPLITBUTTON, 0x1000000
}

const_type! { LVCFMT_I, i32,
	/// [`LVITEM`](crate::LVITEM) `piColFmt` (`i32`).
	=>
	LINE_BREAK, 0x10_0000
	FILL, 0x20_0000
	WRAP, 0x40_0000
	NO_TITLE, 0x80_0000
	TILE_PLACEMENTMASK, Self::LINE_BREAK.0 | Self::FILL.0
}

const_type! { LVFI, u32,
	/// [`LVFINDINFO`](crate::LVFINDINFO) `flags` (`u32`).
	=>
	PARAM, 0x0001
	STRING, 0x0002
	SUBSTRING, 0x0004
	PARTIAL, 0x0008
	WRAP, 0x0020
	NEARESTXY, 0x0040
}

const_type! { LVGIT, u32,
	/// [`NMLVGETINFOTIP`](crate::NMLVGETINFOTIP) `dwFlags` (`u32`).
	=>
	FOLDED, 0x0000
	UNFOLDED, 0x0001
}

const_type! { LVIS, u32,
	/// ListView item
	/// [states](https://docs.microsoft.com/en-us/windows/win32/controls/list-view-item-states)
	/// (`u32`).
	=>
	NONE, 0
	FOCUSED, 0x0001
	SELECTED, 0x0002
	CUT, 0x0004
	DROPHILITED, 0x0008
	GLOW, 0x0010
	ACTIVATING, 0x0020
	OVERLAYMASK, 0x0f00
	STATEIMAGEMASK, 0xf000
}

const_type! { LVI_GROUPID, i32,
	/// [`LVITEM`](crate::LVITEM) `iGroupId` (`i32`).
	=>
	I_GROUPIDCALLBACK, -1
	I_GROUPIDNONE, -2
}

const_type! { LVIF, u32,
	/// [`LVITEM`](crate::LVITEM) `mask` (`u32`).
	=>
	COLFMT, 0x0001_0000
	COLUMNS, 0x0000_0200
	GROUPID, 0x0000_0100
	IMAGE, 0x0000_0002
	INDENT, 0x0000_0010
	NORECOMPUTE, 0x0000_0800
	PARAM, 0x0000_0004
	STATE, 0x0000_0008
	TEXT, 0x0000_0001
}

const_type! { LVKF, u32,
	/// [`NMITEMACTIVATE`](crate::NMITEMACTIVATE) `uKeyFlags` (`u32`).
	=>
	ALT, 0x0001
	CONTROL, 0x0002
	SHIFT, 0x0004
}

const_type_wm! { LVM,
	/// List view control
	/// [messages](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-list-view-control-reference-messages)
	/// (`u32`), convertible to [`WM`](crate::co::WM).
	=>
	FIRST, 0x1000
	=>
	GETBKCOLOR, Self::FIRST.0 + 0
	SETBKCOLOR, Self::FIRST.0 + 1
	GETIMAGELIST, Self::FIRST.0 + 2
	SETIMAGELIST, Self::FIRST.0 + 3
	GETITEMCOUNT, Self::FIRST.0 + 4
	DELETEITEM, Self::FIRST.0 + 8
	DELETEALLITEMS, Self::FIRST.0 + 9
	GETCALLBACKMASK, Self::FIRST.0 + 10
	SETCALLBACKMASK, Self::FIRST.0 + 11
	GETNEXTITEM, Self::FIRST.0 + 12
	GETITEMRECT, Self::FIRST.0 + 14
	SETITEMPOSITION, Self::FIRST.0 + 15
	GETITEMPOSITION, Self::FIRST.0 + 16
	HITTEST, Self::FIRST.0 + 18
	ENSUREVISIBLE, Self::FIRST.0 + 19
	SCROLL, Self::FIRST.0 + 20
	REDRAWITEMS, Self::FIRST.0 + 21
	ARRANGE, Self::FIRST.0 + 22
	GETEDITCONTROL, Self::FIRST.0 + 24
	DELETECOLUMN, Self::FIRST.0 + 28
	GETCOLUMNWIDTH, Self::FIRST.0 + 29
	SETCOLUMNWIDTH, Self::FIRST.0 + 30
	GETHEADER, Self::FIRST.0 + 31
	CREATEDRAGIMAGE, Self::FIRST.0 + 33
	GETVIEWRECT, Self::FIRST.0 + 34
	GETTEXTCOLOR, Self::FIRST.0 + 35
	SETTEXTCOLOR, Self::FIRST.0 + 36
	GETTEXTBKCOLOR, Self::FIRST.0 + 37
	SETTEXTBKCOLOR, Self::FIRST.0 + 38
	GETTOPINDEX, Self::FIRST.0 + 39
	GETCOUNTPERPAGE, Self::FIRST.0 + 40
	GETORIGIN, Self::FIRST.0 + 41
	UPDATE, Self::FIRST.0 + 42
	SETITEMSTATE, Self::FIRST.0 + 43
	GETITEMSTATE, Self::FIRST.0 + 44
	SETITEMCOUNT, Self::FIRST.0 + 47
	SORTITEMS, Self::FIRST.0 + 48
	SETITEMPOSITION32, Self::FIRST.0 + 49
	GETSELECTEDCOUNT, Self::FIRST.0 + 50
	GETITEMSPACING, Self::FIRST.0 + 51
	SETICONSPACING, Self::FIRST.0 + 53
	SETEXTENDEDLISTVIEWSTYLE, Self::FIRST.0 + 54
	GETEXTENDEDLISTVIEWSTYLE, Self::FIRST.0 + 55
	GETSUBITEMRECT, Self::FIRST.0 + 56
	SUBITEMHITTEST, Self::FIRST.0 + 57
	SETCOLUMNORDERARRAY, Self::FIRST.0 + 58
	GETCOLUMNORDERARRAY, Self::FIRST.0 + 59
	SETHOTITEM, Self::FIRST.0 + 60
	GETHOTITEM, Self::FIRST.0 + 61
	SETHOTCURSOR, Self::FIRST.0 + 62
	GETHOTCURSOR, Self::FIRST.0 + 63
	APPROXIMATEVIEWRECT, Self::FIRST.0 + 64
	SETWORKAREAS, Self::FIRST.0 + 65
	GETSELECTIONMARK, Self::FIRST.0 + 66
	SETSELECTIONMARK, Self::FIRST.0 + 67
	GETWORKAREAS, Self::FIRST.0 + 70
	SETHOVERTIME, Self::FIRST.0 + 71
	GETHOVERTIME, Self::FIRST.0 + 72
	GETNUMBEROFWORKAREAS, Self::FIRST.0 + 73
	SETTOOLTIPS, Self::FIRST.0 + 74
	GETITEM, Self::FIRST.0 + 75
	SETITEM, Self::FIRST.0 + 76
	INSERTITEM, Self::FIRST.0 + 77
	GETTOOLTIPS, Self::FIRST.0 + 78
	SORTITEMSEX, Self::FIRST.0 + 81
	FINDITEM, Self::FIRST.0 + 83
	GETSTRINGWIDTH, Self::FIRST.0 + 87
	GETGROUPSTATE, Self::FIRST.0 + 92
	GETFOCUSEDGROUP, Self::FIRST.0 + 93
	GETCOLUMN, Self::FIRST.0 + 95
	SETCOLUMN, Self::FIRST.0 + 96
	INSERTCOLUMN, Self::FIRST.0 + 97
	GETGROUPRECT, Self::FIRST.0 + 98
	GETITEMTEXT, Self::FIRST.0 + 115
	SETITEMTEXT, Self::FIRST.0 + 116
	GETISEARCHSTRING, Self::FIRST.0 + 117
	EDITLABEL, Self::FIRST.0 + 118
	SETBKIMAGE, Self::FIRST.0 + 138
	GETBKIMAGE, Self::FIRST.0 + 139
	SETSELECTEDCOLUMN, Self::FIRST.0 + 140
	SETVIEW, Self::FIRST.0 + 142
	GETVIEW, Self::FIRST.0 + 143
	INSERTGROUP, Self::FIRST.0 + 145
	SETGROUPINFO, Self::FIRST.0 + 147
	GETGROUPINFO, Self::FIRST.0 + 149
	REMOVEGROUP, Self::FIRST.0 + 150
	MOVEGROUP, Self::FIRST.0 + 151
	GETGROUPCOUNT, Self::FIRST.0 + 152
	GETGROUPINFOBYINDEX, Self::FIRST.0 + 153
	MOVEITEMTOGROUP, Self::FIRST.0 + 154
	SETGROUPMETRICS, Self::FIRST.0 + 155
	GETGROUPMETRICS, Self::FIRST.0 + 156
	ENABLEGROUPVIEW, Self::FIRST.0 + 157
	SORTGROUPS, Self::FIRST.0 + 158
	INSERTGROUPSORTED, Self::FIRST.0 + 159
	REMOVEALLGROUPS, Self::FIRST.0 + 160
	HASGROUP, Self::FIRST.0 + 161
	SETTILEVIEWINFO, Self::FIRST.0 + 162
	GETTILEVIEWINFO, Self::FIRST.0 + 163
	SETTILEINFO, Self::FIRST.0 + 164
	GETTILEINFO, Self::FIRST.0 + 165
	SETINSERTMARK, Self::FIRST.0 + 166
	GETINSERTMARK, Self::FIRST.0 + 167
	INSERTMARKHITTEST, Self::FIRST.0 + 168
	GETINSERTMARKRECT, Self::FIRST.0 + 169
	SETINSERTMARKCOLOR, Self::FIRST.0 + 170
	GETINSERTMARKCOLOR, Self::FIRST.0 + 171
	SETINFOTIP, Self::FIRST.0 + 173
	GETSELECTEDCOLUMN, Self::FIRST.0 + 174
	ISGROUPVIEWENABLED, Self::FIRST.0 + 175
	GETOUTLINECOLOR, Self::FIRST.0 + 176
	SETOUTLINECOLOR, Self::FIRST.0 + 177
	CANCELEDITLABEL, Self::FIRST.0 + 179
	MAPINDEXTOID, Self::FIRST.0 + 180
	MAPIDTOINDEX, Self::FIRST.0 + 181
	ISITEMVISIBLE, Self::FIRST.0 + 182
	GETEMPTYTEXT, Self::FIRST.0 + 204
	GETFOOTERRECT, Self::FIRST.0 + 205
	GETFOOTERINFO, Self::FIRST.0 + 206
	GETFOOTERITEMRECT, Self::FIRST.0 + 207
	GETFOOTERITEM, Self::FIRST.0 + 208
	GETITEMINDEXRECT, Self::FIRST.0 + 209
	SETITEMINDEXSTATE, Self::FIRST.0 + 210
	GETNEXTITEMINDEX, Self::FIRST.0 + 211
}

const_type_nm! { LVN,
	/// List view control `WM_NOTIFY`
	/// [notifications](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-list-view-control-reference-notifications)
	/// (`i32`), convertible to [`NM`](crate::co::NM).
	=>
	FIRST, -100
	=>
	ITEMCHANGING, Self::FIRST.0 - 0
	ITEMCHANGED, Self::FIRST.0 - 1
	INSERTITEM, Self::FIRST.0 - 2
	DELETEITEM, Self::FIRST.0 - 3
	DELETEALLITEMS, Self::FIRST.0 - 4
	BEGINLABELEDIT, Self::FIRST.0 - 75
	ENDLABELEDIT, Self::FIRST.0 - 76
	COLUMNCLICK, Self::FIRST.0 - 8
	BEGINDRAG, Self::FIRST.0 - 9
	BEGINRDRAG, Self::FIRST.0 - 11
	ODCACHEHINT, Self::FIRST.0 - 13
	ODFINDITEM, Self::FIRST.0 - 79
	ITEMACTIVATE, Self::FIRST.0 - 14
	ODSTATECHANGED, Self::FIRST.0 - 15
	HOTTRACK, Self::FIRST.0 - 21
	GETDISPINFO, Self::FIRST.0 - 77
	SETDISPINFO, Self::FIRST.0 - 78
	KEYDOWN, Self::FIRST.0 - 55
	MARQUEEBEGIN, Self::FIRST.0 - 56
	GETINFOTIP, Self::FIRST.0 - 58
	INCREMENTALSEARCH, Self::FIRST.0 - 63
	COLUMNDROPDOWN, Self::FIRST.0 - 64
	COLUMNOVERFLOWCLICK, Self::FIRST.0 - 66
	BEGINSCROLL, Self::FIRST.0 - 80
	ENDSCROLL, Self::FIRST.0 - 81
	LINKCLICK, Self::FIRST.0 - 84
	GETEMPTYMARKUP, Self::FIRST.0 - 87
}

const_type! { LVNI, u32,
	/// [`LVM_GETNEXTITEM`](crate::msg::lvm::GetNextItem) relationship (`u32`).
	=>
	ALL, 0x0000
	FOCUSED, 0x0001
	SELECTED, 0x0002
	CUT, 0x0004
	DROPHILITED, 0x0008
	VISIBLEORDER, 0x0010
	PREVIOUS, 0x0020
	VISIBLEONLY, 0x0040
	SAMEGROUPONLY, 0x0080
	ABOVE, 0x0100
	BELOW, 0x0200
	TOLEFT, 0x0400
	TORIGHT, 0x0800
}

const_type_ws! { LVS,
	/// List view control
	/// [styles](https://docs.microsoft.com/en-us/windows/win32/controls/list-view-window-styles)
	/// (`u32`), convertible to [`WS`](crate::co::WS).
	=>
	ICON, 0x0000
	REPORT, 0x0001
	SMALLICON, 0x0002
	LIST, 0x0003
	TYPEMASK, 0x0003
	SINGLESEL, 0x0004
	SHOWSELALWAYS, 0x0008
	SORTASCENDING, 0x0010
	SORTDESCENDING, 0x0020
	SHAREIMAGELISTS, 0x0040
	NOLABELWRAP, 0x0080
	AUTOARRANGE, 0x0100
	EDITLABELS, 0x0200
	OWNERDATA, 0x1000
	NOSCROLL, 0x2000
	TYPESTYLEMASK, 0xfc00
	ALIGNTOP, 0x0000
	ALIGNLEFT, 0x0800
	ALIGNMASK, 0x0c00
	OWNERDRAWFIXED, 0x0400
	NOCOLUMNHEADER, 0x4000
	NOSORTHEADER, 0x8000
}

const_type_wsex! { LVS_EX,
	/// Extended list view control
	/// [styles](https://docs.microsoft.com/en-us/windows/win32/controls/extended-list-view-styles)
	/// (`u32`), convertible to [`WS_EX`](crate::co::WS_EX).
	=>
	NONE, 0
	AUTOAUTOARRANGE, 0x0100_0000
	AUTOCHECKSELECT, 0x0800_0000
	AUTOSIZECOLUMNS, 0x1000_0000
	BORDERSELECT, 0x0000_8000
	CHECKBOXES, 0x0000_0004
	COLUMNOVERFLOW, 0x8000_0000
	COLUMNSNAPPOINTS, 0x4000_0000
	DOUBLEBUFFER, 0x0001_0000
	FLATSB, 0x0000_0100
	FULLROWSELECT, 0x0000_0020
	GRIDLINES, 0x0000_0001
	HEADERDRAGDROP, 0x0000_0010
	HEADERINALLVIEWS, 0x0200_0000
	HIDELABELS, 0x0002_0000
	INFOTIP, 0x0000_0400
	JUSTIFYCOLUMNS, 0x0020_0000
	LABELTIP, 0x0000_4000
	MULTIWORKAREAS, 0x0000_2000
	ONECLICKACTIVATE, 0x0000_0040
	REGIONAL, 0x0000_0200
	SIMPLESELECT, 0x0010_0000
	SINGLEROW, 0x0004_0000
	SNAPTOGRID, 0x0008_0000
	SUBITEMIMAGES, 0x0000_0002
	TRACKSELECT, 0x0000_0008
	TRANSPARENTBKGND, 0x0040_0000
	TRANSPARENTSHADOWTEXT, 0x0080_0000
	TWOCLICKACTIVATE, 0x0000_0080
	UNDERLINECOLD, 0x000_01000
	UNDERLINEHOT, 0x0000_0800
}

const_type_ws! { LWS,
	/// SysLink control
	/// [styles](https://docs.microsoft.com/en-us/windows/win32/controls/syslink-control-styles)
	/// (`u32`), convertible to [`WS`](crate::co::WS).
	=>
	/// The background mix mode is transparent.
	TRANSPARENT, 0x0001
	/// When the link has keyboard focus and the user presses Enter, the
	/// keystroke is ignored by the control and passed to the host dialog box.
	IGNORERETURN, 0x0002
	/// If the text contains an ampersand, it is treated as a literal character
	/// rather than the prefix to a shortcut key.
	NOPREFIX, 0x0004
	/// The link is displayed in the current visual style.
	USEVISUALSTYLE, 0x0008
	/// An `NM_CUSTOMTEXT` notification is sent when the control is drawn, so
	/// that the application can supply text dynamically.
	USECUSTOMTEXT, 0x0010
	/// The text is right-justified.
	RIGHT, 0x0020
}
