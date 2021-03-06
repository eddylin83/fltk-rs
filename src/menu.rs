use crate::image::Image;
pub use crate::prelude::*;
use fltk_sys::menu::*;
use std::{
    ffi::{CStr, CString},
    mem,
    os::raw,
};

/// Creates a menu bar
#[derive(WidgetExt, MenuExt, Debug)]
pub struct MenuBar {
    _inner: *mut Fl_Menu_Bar,
}

/// Creates a menu button
#[derive(WidgetExt, MenuExt, Debug)]
pub struct MenuButton {
    _inner: *mut Fl_Menu_Button,
}

/// Creates a menu choice
#[derive(WidgetExt, MenuExt, Debug)]
pub struct Choice {
    _inner: *mut Fl_Choice,
}

/// Creates a menu item
#[derive(Debug, Clone)]
pub struct MenuItem {
    _inner: *mut Fl_Menu_Item,
}

/// Defines the menu flag for any added menu items using the add() method
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MenuFlag {
    Normal = 0,
    Inactive = 1,
    Toggle = 2,
    Value = 4,
    Radio = 8,
    Invisible = 0x10,
    SubmenuPointer = 0x20,
    Submenu = 0x40,
    MenuDivider = 0x80,
    MenuHorizontal = 0x100,
}

impl MenuItem {
    /// Initializes a new window, useful for popup menus
    pub fn new(choices: Vec<&str>) -> MenuItem {
        unsafe {
            let sz = choices.len();
            let mut temp: Vec<*mut raw::c_char> = vec![];
            for choice in choices {
                temp.push(CString::new(choice).unwrap().into_raw());
            }
            let item_ptr = Fl_Menu_Item_new(temp.as_ptr() as *mut *mut raw::c_char, sz as i32);
            assert!(!item_ptr.is_null());
            MenuItem { _inner: item_ptr }
        }
    }
    
    /// Creates a popup menu at the specified coordinates and returns its choice
    pub fn popup(&mut self, x: i32, y: i32) -> Option<MenuItem> {
        if self._inner.is_null() {
            return None;
        }
        unsafe {
            let item = Fl_Menu_Item_popup(self._inner, x, y);
            if item.is_null() {
                None
            } else {
                let item = MenuItem {
                    _inner: item as *mut Fl_Menu_Item,
                };
                Some(item)
            }
        }
    }
    
    /// Returns the label of the menu item
    pub fn label(&self) -> Option<String> {
        if self._inner.is_null() {
            return None;
        }
        unsafe {
            let label_ptr = Fl_Menu_Item_label(self._inner);
            if label_ptr.is_null() {
                return None;
            }
            Some(CStr::from_ptr(label_ptr as *mut raw::c_char)
                .to_string_lossy()
                .to_string())
        }
    }

    /// Sets the label of the menu item
    pub fn set_label(&mut self, txt: &str) {
        assert!(!self._inner.is_null());
        unsafe {
            let txt = CString::new(txt).unwrap();
            Fl_Menu_Item_set_label(self._inner, txt.as_ptr());
        }
    }

    /// Returns the label type of the menu item
    pub fn label_type<T: WidgetType>(&self) -> T {
        assert!(!self._inner.is_null());
        unsafe { T::from_i32(Fl_Menu_Item_label_type(self._inner)) }
    }

    /// Sets the label type of the menu item
    pub fn set_label_type<T: WidgetType>(&mut self, typ: T) {
        assert!(!self._inner.is_null());
        unsafe {
            Fl_Menu_Item_set_label_type(self._inner, typ.to_int());
        }
    }

    /// Returns the label color of the menu item
    pub fn label_color(&self) -> Color {
        assert!(!self._inner.is_null());
        unsafe { mem::transmute(Fl_Menu_Item_label_color(self._inner)) }
    }

    /// Sets the label color of the menu item
    pub fn set_label_color(&mut self, color: Color) {
        assert!(!self._inner.is_null());
        unsafe { Fl_Menu_Item_set_label_color(self._inner, color as u32) }
    }

    /// Returns the label font of the menu item
    pub fn label_font(&self) -> Font {
        assert!(!self._inner.is_null());
        unsafe { mem::transmute(Fl_Menu_Item_label_font(self._inner)) }
    }

    /// Sets the label font of the menu item
    pub fn set_label_font(&mut self, font: Font) {
        assert!(!self._inner.is_null());
        unsafe { Fl_Menu_Item_set_label_font(self._inner, font as i32) }
    }

    /// Returns the label size of the menu item
    pub fn label_size(&self) -> u32 {
        assert!(!self._inner.is_null());
        unsafe { Fl_Menu_Item_label_size(self._inner) as u32 }
    }

    /// Sets the label size of the menu item
    pub fn set_label_size(&mut self, sz: u32) {
        assert!(!self._inner.is_null());
        unsafe { Fl_Menu_Item_set_label_size(self._inner, sz as i32) }
    }

    /// Returns the value of the menu item
    pub fn value(&self) -> bool {
        assert!(!self._inner.is_null());
        unsafe {
            match Fl_Menu_Item_value(self._inner) {
                0 => false,
                _ => true,
            }
        }
    }

    /// Sets the menu item
    pub fn set(&mut self) {
        assert!(!self._inner.is_null());
        unsafe { Fl_Menu_Item_set(self._inner) }
    }

    /// Clears the menu item
    pub fn clear(&mut self) {
        assert!(!self._inner.is_null());
        unsafe { Fl_Menu_Item_clear(self._inner) }
    }

    /// Returns whether the menu item is visible or not
    pub fn visible(&self) -> bool {
        assert!(!self._inner.is_null());
        unsafe {
            match Fl_Menu_Item_visible(self._inner) {
                0 => false,
                _ => true,
            }
        }
    }

    /// Returns whether the menu item is active
    pub fn active(&mut self) -> bool {
        assert!(!self._inner.is_null());
        unsafe {
            match Fl_Menu_Item_active(self._inner) {
                0 => false,
                _ => true,
            }
        }
    }

    /// Activates the menu item
    pub fn activate(&mut self) {
        assert!(!self._inner.is_null());
        unsafe { Fl_Menu_Item_activate(self._inner) }
    }

    /// Deactivates the menu item
    pub fn deactivate(&mut self) {
        assert!(!self._inner.is_null());
        unsafe { Fl_Menu_Item_deactivate(self._inner) }
    }

    /// Shows the menu item
    pub fn show(&mut self) {
        assert!(!self._inner.is_null());
        unsafe { Fl_Menu_Item_show(self._inner) }
    }

    /// Hides the menu item
    pub fn hide(&mut self) {
        assert!(!self._inner.is_null());
        unsafe { Fl_Menu_Item_hide(self._inner) }
    }
}

unsafe impl Send for MenuItem {}

unsafe impl Sync for MenuItem {}

#[cfg(test)]
mod menu {
    use super::*;
    #[test]
    fn label() {
        let mut menu = MenuBar::new(0, 0, 0, 0, "hello");
        menu.set_label("cloned");
    }
    #[test]
    fn tooltip() {
        let mut menu = MenuBar::new(0, 0, 0, 0, "hello");
        menu.set_tooltip("tooltip");
        assert!(menu.tooltip().unwrap() == "tooltip");
    }
}
