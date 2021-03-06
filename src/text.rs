use crate::image::Image;
pub use crate::prelude::*;
use fltk_sys::text::*;
use std::{
    ffi::{CStr, CString},
    mem,
    os::raw,
};

/// Wraps a text buffer, Cloning a text buffer invalidates the underlying pointer, thus the no derive(Clone)
#[derive(Debug)]
pub struct TextBuffer {
    _inner: *mut Fl_Text_Buffer,
}

impl TextBuffer {
    /// Initialized a default text buffer
    pub fn default() -> Self {
        unsafe {
            let text_buffer = Fl_Text_Buffer_new();
            assert!(!text_buffer.is_null());
            TextBuffer {
                _inner: text_buffer,
            }
        }
    }

    /// Deletes the TextBuffer
    pub unsafe fn delete(&mut self) {
        Fl_Text_Buffer_delete(self._inner)
    }

    /// Initialized a text buffer from a pointer
    pub unsafe fn from_ptr(ptr: *mut Fl_Text_Buffer) -> Self {
        TextBuffer { _inner: ptr }
    }

    /// Returns the inner pointer from a text buffer
    pub unsafe fn as_ptr(&self) -> *mut Fl_Text_Buffer {
        self._inner
    }

    /// Sets the text of the buffer
    pub fn set_text(&mut self, txt: &str) {
        unsafe {
            let txt = CString::new(txt).unwrap();
            Fl_Text_Buffer_set_text(self._inner, txt.as_ptr())
        }
    }

    /// Returns the text of the buffer
    pub fn text(&self) -> String {
        unsafe {
            let text = Fl_Text_Buffer_text(self._inner);
            assert!(!text.is_null());
            CString::from_raw(text as *mut raw::c_char)
                .to_string_lossy()
                .to_string()
        }
    }

    /// Appends to the buffer
    pub fn append(&mut self, text: &str) {
        let text = CString::new(text).unwrap();
        unsafe { Fl_Text_Buffer_append(self._inner, text.as_ptr()) }
    }

    /// Get the length of the buffer
    pub fn length(&self) -> u32 {
        unsafe { Fl_Text_Buffer_length(self._inner) as u32 }
    }

    /// Removes from the buffer
    pub fn remove(&mut self, start: u32, end: u32) {
        debug_assert!(
            start <= std::i32::MAX as u32,
            "u32 entries must be < std::i32::MAX for compatibility!"
        );
        debug_assert!(
            end <= std::i32::MAX as u32,
            "u32 entries must be < std::i32::MAX for compatibility!"
        );
        unsafe {
            Fl_Text_Buffer_remove(self._inner, start as i32, end as i32);
        }
    }

    /// Returns the text within the range
    pub fn text_range(&self, start: u32, end: u32) -> Option<String> {
        debug_assert!(
            start <= std::i32::MAX as u32,
            "u32 entries must be < std::i32::MAX for compatibility!"
        );
        debug_assert!(
            end <= std::i32::MAX as u32,
            "u32 entries must be < std::i32::MAX for compatibility!"
        );
        unsafe {
            let x = Fl_Text_Buffer_text_range(self._inner, start as i32, end as i32);
            if x.is_null() {
                None
            } else {
                Some(
                    CString::from_raw(x as *mut raw::c_char)
                        .to_string_lossy()
                        .to_string(),
                )
            }
        }
    }

    /// Inserts text into a position
    pub fn insert(&mut self, pos: u32, text: &str) {
        debug_assert!(
            pos <= std::i32::MAX as u32,
            "u32 entries must be < std::i32::MAX for compatibility!"
        );
        let text = CString::new(text).unwrap();
        unsafe { Fl_Text_Buffer_insert(self._inner, pos as i32, text.as_ptr()) }
    }

    /// Replaces text from position ```start``` to ```end```
    pub fn replace(&mut self, start: u32, end: u32, text: &str) {
        debug_assert!(
            start <= std::i32::MAX as u32,
            "u32 entries must be < std::i32::MAX for compatibility!"
        );
        debug_assert!(
            end <= std::i32::MAX as u32,
            "u32 entries must be < std::i32::MAX for compatibility!"
        );
        let text = CString::new(text).unwrap();
        unsafe { Fl_Text_Buffer_replace(self._inner, start as i32, end as i32, text.as_ptr()) }
    }

    /// Copies text from a source buffer into the current buffer
    pub fn copy(&mut self, source_buf: &TextBuffer, start: u32, end: u32, to: u32) {
        debug_assert!(
            start <= std::i32::MAX as u32,
            "u32 entries must be < std::i32::MAX for compatibility!"
        );
        debug_assert!(
            end <= std::i32::MAX as u32,
            "u32 entries must be < std::i32::MAX for compatibility!"
        );
        debug_assert!(
            to <= std::i32::MAX as u32,
            "u32 entries must be < std::i32::MAX for compatibility!"
        );
        unsafe {
            Fl_Text_Buffer_copy(
                self._inner,
                source_buf.as_ptr(),
                start as i32,
                end as i32,
                to as i32,
            )
        }
    }

    /// Performs an undo operation on the buffer
    pub fn undo(&mut self) -> Result<(), FltkError> {
        unsafe {
            match Fl_Text_Buffer_undo(self._inner, std::ptr::null_mut()) {
                0 => Err(FltkError::Unknown(String::from("Failed to undo"))),
                _ => Ok(()),
            }
        }
    }

    /// Sets whether the buffer can undo
    pub fn can_undo(&mut self, flag: bool) {
        unsafe { Fl_Text_Buffer_canUndo(self._inner, flag as i8) }
    }

    /// Loads a file into the buffer
    pub fn load_file(&mut self, path: &std::path::Path) -> Result<(), FltkError> {
        if !path.exists() {
            return Err(FltkError::Internal(FltkErrorKind::ResourceNotFound));
        }
        let path = path.to_str().unwrap();
        let path = CString::new(path)?;
        unsafe {
            match Fl_Text_Buffer_loadfile(self._inner, path.as_ptr(), 0) {
                0 => Err(FltkError::Internal(FltkErrorKind::ResourceNotFound)),
                _ => Ok(()),
            }
        }
    }

    /// Returns the tab distance for the buffer
    pub fn tab_distance(&self) -> u32 {
        unsafe { Fl_Text_Buffer_tab_distance(self._inner) as u32 }
    }

    /// Sets the tab distance
    pub fn set_tab_distance(&mut self, tab_dist: u32) {
        debug_assert!(
            tab_dist <= std::i32::MAX as u32,
            "u32 entries must be < std::i32::MAX for compatibility!"
        );
        unsafe { Fl_Text_Buffer_set_tab_distance(self._inner, tab_dist as i32) }
    }

    /// Selects the text from start to end
    pub fn select(&mut self, start: u32, end: u32) {
        debug_assert!(
            start <= std::i32::MAX as u32,
            "u32 entries must be < std::i32::MAX for compatibility!"
        );
        debug_assert!(
            end <= std::i32::MAX as u32,
            "u32 entries must be < std::i32::MAX for compatibility!"
        );
        unsafe { Fl_Text_Buffer_select(self._inner, start as i32, end as i32) }
    }

    /// Returns whether text is selected
    pub fn selected(&self) -> bool {
        unsafe {
            match Fl_Text_Buffer_selected(self._inner) {
                0 => false,
                _ => true,
            }
        }
    }

    /// Unselects text
    pub fn unselect(&mut self) {
        unsafe { Fl_Text_Buffer_unselect(self._inner) }
    }

    /// Returns the selection position
    pub fn selection_position(&mut self) -> Option<(u32, u32)> {
        unsafe {
            let start: *mut raw::c_int = std::ptr::null_mut();
            let end: *mut raw::c_int = std::ptr::null_mut();
            let ret = Fl_Text_Buffer_selection_position(self._inner, start, end);
            if ret != 0 {
                let x = (*start as u32, *end as u32);
                Some(x)
            } else {
                None
            }
        }
    }

    /// Returns the selection text
    pub fn selection_text(&mut self) -> String {
        unsafe {
            let x = Fl_Text_Buffer_selection_text(self._inner);
            assert!(!x.is_null());
            CString::from_raw(x as *mut raw::c_char)
                .to_string_lossy()
                .to_string()
        }
    }

    /// Removes the selection
    pub fn remove_selection(&mut self) {
        unsafe { Fl_Text_Buffer_remove_selection(self._inner) }
    }

    /// Replaces selection
    pub fn replace_selection(&mut self, text: &str) {
        let text = CString::new(text).unwrap();
        unsafe { Fl_Text_Buffer_replace_selection(self._inner, text.as_ptr()) }
    }

    /// Highlights selection
    pub fn highlight(&mut self, start: u32, end: u32) {
        debug_assert!(
            start <= std::i32::MAX as u32,
            "u32 entries must be < std::i32::MAX for compatibility!"
        );
        debug_assert!(
            end <= std::i32::MAX as u32,
            "u32 entries must be < std::i32::MAX for compatibility!"
        );
        unsafe { Fl_Text_Buffer_highlight(self._inner, start as i32, end as i32) }
    }

    /// Returns whether text is highlighted
    pub fn is_highlighted(&mut self) -> bool {
        unsafe {
            match Fl_Text_Buffer_is_highlighted(self._inner) {
                0 => false,
                _ => true,
            }
        }
    }

    /// Unhighlights text
    pub fn unhighlight(&mut self) {
        unsafe { Fl_Text_Buffer_unhighlight(self._inner) }
    }

    /// Returns the highlight position
    pub fn highlight_position(&mut self) -> Option<(u32, u32)> {
        unsafe {
            let start: *mut raw::c_int = std::ptr::null_mut();
            let end: *mut raw::c_int = std::ptr::null_mut();
            let ret = Fl_Text_Buffer_highlight_position(self._inner, start, end);
            if ret != 0 {
                let x = (*start as u32, *end as u32);
                Some(x)
            } else {
                None
            }
        }
    }

    /// Returns the highlighted text
    pub fn highlight_text(&mut self) -> String {
        unsafe {
            let x = Fl_Text_Buffer_highlight_text(self._inner);
            assert!(!x.is_null());
            CString::from_raw(x as *mut raw::c_char)
                .to_string_lossy()
                .to_string()
        }
    }

    /// Returns the line at pos
    pub fn line_text(&self, pos: u32) -> String {
        debug_assert!(
            pos <= std::i32::MAX as u32,
            "u32 entries must be < std::i32::MAX for compatibility!"
        );
        unsafe {
            let x = Fl_Text_Buffer_line_text(self._inner, pos as i32);
            assert!(!x.is_null());
            CString::from_raw(x as *mut raw::c_char)
                .to_string_lossy()
                .to_string()
        }
    }

    /// Returns the index of the line's start position at pos
    pub fn line_start(&self, pos: u32) -> u32 {
        debug_assert!(
            pos <= std::i32::MAX as u32,
            "u32 entries must be < std::i32::MAX for compatibility!"
        );
        unsafe { Fl_Text_Buffer_line_start(self._inner, pos as i32) as u32 }
    }

    /// Returns the index of the first character of a word at pos
    pub fn word_start(&self, pos: u32) -> u32 {
        debug_assert!(
            pos <= std::i32::MAX as u32,
            "u32 entries must be < std::i32::MAX for compatibility!"
        );
        unsafe { Fl_Text_Buffer_word_start(self._inner, pos as i32) as u32 }
    }

    /// Returns the index of the last character of a word at pos
    pub fn word_end(&self, pos: u32) -> u32 {
        debug_assert!(
            pos <= std::i32::MAX as u32,
            "u32 entries must be < std::i32::MAX for compatibility!"
        );
        unsafe { Fl_Text_Buffer_word_end(self._inner, pos as i32) as u32 }
    }

    /// Counts the lines from start to end
    pub fn count_lines(&self, start: u32, end: u32) -> u32 {
        debug_assert!(
            start <= std::i32::MAX as u32,
            "u32 entries must be < std::i32::MAX for compatibility!"
        );
        debug_assert!(
            end <= std::i32::MAX as u32,
            "u32 entries must be < std::i32::MAX for compatibility!"
        );
        unsafe { Fl_Text_Buffer_count_lines(self._inner, start as i32, end as i32) as u32 }
    }

    /// Calls the modify callbacks
    pub fn call_modify_callbacks(&mut self) {
        unsafe { Fl_Text_Buffer_call_modify_callbacks(self._inner) }
    }

    /// Adds a modify callback
    pub fn add_modify_callback(&mut self, cb: Box<dyn FnMut(u32, u32, u32, u32, &str)>) {
        unsafe {
            unsafe extern "C" fn shim(
                pos: raw::c_int,
                inserted: raw::c_int,
                deleted: raw::c_int,
                restyled: raw::c_int,
                deleted_text: *const raw::c_char,
                data: *mut raw::c_void,
            ) {
                let mut temp = String::from("");
                if !deleted_text.is_null() {
                    temp = CStr::from_ptr(deleted_text).to_string_lossy().to_string();
                }
                let a: *mut Box<dyn FnMut(u32, u32, u32, u32, &str)> = mem::transmute(data);
                let f: &mut (dyn FnMut(u32, u32, u32, u32, &str)) = &mut **a;
                f(
                    pos as u32,
                    inserted as u32,
                    deleted as u32,
                    restyled as u32,
                    &temp,
                )
            }
            let a: *mut Box<dyn FnMut(u32, u32, u32, u32, &str)> = Box::into_raw(Box::new(cb));
            let data: *mut raw::c_void = mem::transmute(a);
            let callback: Fl_Text_Modify_Cb = Some(shim);
            Fl_Text_Buffer_add_modify_callback(self._inner, callback, data);
        }
    }

    /// Removes a modify callback
    pub fn remove_modify_callback(&mut self, cb: Box<dyn FnMut(u32, u32, u32, u32, &str)>) {
        unsafe {
            unsafe extern "C" fn shim(
                pos: raw::c_int,
                inserted: raw::c_int,
                deleted: raw::c_int,
                restyled: raw::c_int,
                deleted_text: *const raw::c_char,
                data: *mut raw::c_void,
            ) {
                let mut temp = String::from("");
                if !deleted_text.is_null() {
                    temp = CStr::from_ptr(deleted_text).to_string_lossy().to_string();
                }
                let a: *mut Box<dyn FnMut(u32, u32, u32, u32, &str)> = mem::transmute(data);
                let f: &mut (dyn FnMut(u32, u32, u32, u32, &str)) = &mut **a;
                f(
                    pos as u32,
                    inserted as u32,
                    deleted as u32,
                    restyled as u32,
                    &temp,
                )
            }
            let a: *mut Box<dyn FnMut(u32, u32, u32, u32, &str)> = Box::into_raw(Box::new(cb));
            let data: *mut raw::c_void = mem::transmute(a);
            let callback: Fl_Text_Modify_Cb = Some(shim);
            Fl_Text_Buffer_remove_modify_callback(self._inner, callback, data);
        }
    }
}

unsafe impl Sync for TextBuffer {}
unsafe impl Send for TextBuffer {}

impl Clone for TextBuffer {
    fn clone(&self) -> TextBuffer {
        let mut temp = TextBuffer::default();
        temp.copy(self, 0, 0, self.length());
        temp
    }
}

// impl Drop for TextBuffer {
//     fn drop(&mut self) {
//         unsafe { Fl_Text_Buffer_delete(self._inner) }
//     }
// }

/// Creates a non-editable text display widget
#[derive(WidgetExt, DisplayExt, Debug)]
pub struct TextDisplay {
    _inner: *mut Fl_Text_Display,
}

/// Creates an editable text display widget
#[derive(WidgetExt, DisplayExt, Debug)]
pub struct TextEditor {
    _inner: *mut Fl_Text_Editor,
}

/// Creates an editable text display widget
#[derive(WidgetExt, DisplayExt, Debug)]
pub struct SimpleTerminal {
    _inner: *mut Fl_Simple_Terminal,
}

#[derive(Debug, Clone, Copy)]
pub struct StyleTableEntry {
    pub color: Color,
    pub font: Font,
    pub size: u32,
}

impl TextEditor {
    /// Create an new TextEditor widget
    pub fn new(x: i32, y: i32, w: i32, h: i32, buf: &mut TextBuffer) -> TextEditor {
        let temp = CString::new("").unwrap();
        unsafe {
            let text_editor = Fl_Text_Editor_new(x, y, w, h, temp.into_raw() as *const raw::c_char);
            assert!(!text_editor.is_null());
            let mut x = TextEditor {
                _inner: text_editor,
            };
            x.set_buffer(buf);
            x
        }
    }

    /// Creates a default and zero initialized TextEditor
    pub fn default(buf: &mut TextBuffer) -> TextEditor {
        let temp = CString::new("").unwrap();
        unsafe {
            let text_editor = Fl_Text_Editor_new(0, 0, 0, 0, temp.into_raw() as *const raw::c_char);
            assert!(!text_editor.is_null());
            let mut x = TextEditor {
                _inner: text_editor,
            };
            x.set_buffer(buf);
            x
        }
    }

    /// Copies the text within the TextEditor widget
    pub fn copy(&self) {
        unsafe {
            kf_copy(self._inner);
        }
    }

    /// Cuts the text within the TextEditor widget
    pub fn cut(&self) {
        unsafe {
            kf_cut(self._inner);
        }
    }

    /// Pastes text from the clipboard into the TextEditor widget
    pub fn paste(&self) {
        unsafe {
            kf_paste(self._inner);
        }
    }

    /// Undo changes in the TextEditor widget
    pub fn undo(&self) {
        unsafe {
            kf_undo(self._inner);
        }
    }
}

impl TextDisplay {
    /// Create an new TextDisplay widget
    pub fn new(x: i32, y: i32, w: i32, h: i32, buf: &mut TextBuffer) -> TextDisplay {
        let temp = CString::new("").unwrap();
        unsafe {
            let text_display =
                Fl_Text_Display_new(x, y, w, h, temp.into_raw() as *const raw::c_char);
            assert!(!text_display.is_null(),);
            let mut x = TextDisplay {
                _inner: text_display,
            };
            x.set_buffer(buf);
            x
        }
    }

    /// Creates a default and zero initialized TextDisplay
    pub fn default(buf: &mut TextBuffer) -> TextDisplay {
        let temp = CString::new("").unwrap();
        unsafe {
            let text_display =
                Fl_Text_Display_new(0, 0, 0, 0, temp.into_raw() as *const raw::c_char);
            assert!(!text_display.is_null(),);
            let mut x = TextDisplay {
                _inner: text_display,
            };
            x.set_buffer(buf);
            x
        }
    }
}

impl SimpleTerminal {
    /// Create an new SimpleTerminal widget
    pub fn new(x: i32, y: i32, w: i32, h: i32, buf: &mut TextBuffer) -> SimpleTerminal {
        let temp = CString::new("").unwrap();
        unsafe {
            let simple_terminal =
                Fl_Simple_Terminal_new(x, y, w, h, temp.into_raw() as *const raw::c_char);
            assert!(!simple_terminal.is_null(),);
            let mut x = SimpleTerminal {
                _inner: simple_terminal,
            };
            x.set_buffer(buf);
            x
        }
    }

    /// Creates a default and zero initialized SimpleTerminal
    pub fn default(buf: &mut TextBuffer) -> SimpleTerminal {
        let temp = CString::new("").unwrap();
        unsafe {
            let simple_terminal =
                Fl_Simple_Terminal_new(0, 0, 0, 0, temp.into_raw() as *const raw::c_char);
            assert!(!simple_terminal.is_null(),);
            let mut x = SimpleTerminal {
                _inner: simple_terminal,
            };
            x.set_buffer(buf);
            x
        }
    }
}

#[cfg(test)]
mod editor {
    #[test]
    fn buffer() {}
}
