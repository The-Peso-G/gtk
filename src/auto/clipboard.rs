// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use SelectionData;
use TextBuffer;
use ffi;
use gdk;
use gdk_pixbuf;
use glib;
use glib::GString;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct Clipboard(Object<ffi::GtkClipboard, ClipboardClass>);

    match fn {
        get_type => || ffi::gtk_clipboard_get_type(),
    }
}

impl Clipboard {
    pub fn get(selection: &gdk::Atom) -> Clipboard {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gtk_clipboard_get(selection.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    pub fn get_default<P: IsA<gdk::Display>>(display: &P) -> Option<Clipboard> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gtk_clipboard_get_default(display.as_ref().to_glib_none().0))
        }
    }

    pub fn get_for_display<P: IsA<gdk::Display>>(display: &P, selection: &gdk::Atom) -> Clipboard {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gtk_clipboard_get_for_display(display.as_ref().to_glib_none().0, selection.to_glib_none().0))
        }
    }
}

pub const NONE_CLIPBOARD: Option<&Clipboard> = None;

pub trait ClipboardExt: 'static {
    fn clear(&self);

    fn get_display(&self) -> Option<gdk::Display>;

    fn get_owner(&self) -> Option<glib::Object>;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_selection(&self) -> Option<gdk::Atom>;

    //fn request_contents(&self, target: &gdk::Atom, callback: /*Unknown conversion*//*Unimplemented*/ClipboardReceivedFunc);

    //fn request_image(&self, callback: /*Unknown conversion*//*Unimplemented*/ClipboardImageReceivedFunc);

    //fn request_rich_text<P: IsA<TextBuffer>>(&self, buffer: &P, callback: /*Unknown conversion*//*Unimplemented*/ClipboardRichTextReceivedFunc);

    //fn request_text(&self, callback: /*Unknown conversion*//*Unimplemented*/ClipboardTextReceivedFunc);

    fn set_image<P: IsA<gdk_pixbuf::Pixbuf>>(&self, pixbuf: &P);

    fn set_text(&self, text: &str);

    fn store(&self);

    fn wait_for_contents(&self, target: &gdk::Atom) -> Option<SelectionData>;

    fn wait_for_image(&self) -> Option<gdk_pixbuf::Pixbuf>;

    fn wait_for_rich_text<P: IsA<TextBuffer>>(&self, buffer: &P) -> (Vec<u8>, gdk::Atom);

    fn wait_for_targets(&self) -> Option<Vec<gdk::Atom>>;

    fn wait_for_text(&self) -> Option<GString>;

    fn wait_for_uris(&self) -> Vec<GString>;

    fn wait_is_image_available(&self) -> bool;

    fn wait_is_rich_text_available<P: IsA<TextBuffer>>(&self, buffer: &P) -> bool;

    fn wait_is_target_available(&self, target: &gdk::Atom) -> bool;

    fn wait_is_text_available(&self) -> bool;

    fn wait_is_uris_available(&self) -> bool;

    //fn connect_owner_change<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Clipboard>> ClipboardExt for O {
    fn clear(&self) {
        unsafe {
            ffi::gtk_clipboard_clear(self.as_ref().to_glib_none().0);
        }
    }

    fn get_display(&self) -> Option<gdk::Display> {
        unsafe {
            from_glib_none(ffi::gtk_clipboard_get_display(self.as_ref().to_glib_none().0))
        }
    }

    fn get_owner(&self) -> Option<glib::Object> {
        unsafe {
            from_glib_none(ffi::gtk_clipboard_get_owner(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_selection(&self) -> Option<gdk::Atom> {
        unsafe {
            from_glib_none(ffi::gtk_clipboard_get_selection(self.as_ref().to_glib_none().0))
        }
    }

    //fn request_contents(&self, target: &gdk::Atom, callback: /*Unknown conversion*//*Unimplemented*/ClipboardReceivedFunc) {
    //    unsafe { TODO: call ffi::gtk_clipboard_request_contents() }
    //}

    //fn request_image(&self, callback: /*Unknown conversion*//*Unimplemented*/ClipboardImageReceivedFunc) {
    //    unsafe { TODO: call ffi::gtk_clipboard_request_image() }
    //}

    //fn request_rich_text<P: IsA<TextBuffer>>(&self, buffer: &P, callback: /*Unknown conversion*//*Unimplemented*/ClipboardRichTextReceivedFunc) {
    //    unsafe { TODO: call ffi::gtk_clipboard_request_rich_text() }
    //}

    //fn request_text(&self, callback: /*Unknown conversion*//*Unimplemented*/ClipboardTextReceivedFunc) {
    //    unsafe { TODO: call ffi::gtk_clipboard_request_text() }
    //}

    fn set_image<P: IsA<gdk_pixbuf::Pixbuf>>(&self, pixbuf: &P) {
        unsafe {
            ffi::gtk_clipboard_set_image(self.as_ref().to_glib_none().0, pixbuf.as_ref().to_glib_none().0);
        }
    }

    fn set_text(&self, text: &str) {
        let len = text.len() as i32;
        unsafe {
            ffi::gtk_clipboard_set_text(self.as_ref().to_glib_none().0, text.to_glib_none().0, len);
        }
    }

    fn store(&self) {
        unsafe {
            ffi::gtk_clipboard_store(self.as_ref().to_glib_none().0);
        }
    }

    fn wait_for_contents(&self, target: &gdk::Atom) -> Option<SelectionData> {
        unsafe {
            from_glib_full(ffi::gtk_clipboard_wait_for_contents(self.as_ref().to_glib_none().0, target.to_glib_none().0))
        }
    }

    fn wait_for_image(&self) -> Option<gdk_pixbuf::Pixbuf> {
        unsafe {
            from_glib_full(ffi::gtk_clipboard_wait_for_image(self.as_ref().to_glib_none().0))
        }
    }

    fn wait_for_rich_text<P: IsA<TextBuffer>>(&self, buffer: &P) -> (Vec<u8>, gdk::Atom) {
        unsafe {
            let mut format = gdk::Atom::uninitialized();
            let mut length = mem::uninitialized();
            let ret = FromGlibContainer::from_glib_full_num(ffi::gtk_clipboard_wait_for_rich_text(self.as_ref().to_glib_none().0, buffer.as_ref().to_glib_none().0, format.to_glib_none_mut().0, &mut length), length as usize);
            (ret, format)
        }
    }

    fn wait_for_targets(&self) -> Option<Vec<gdk::Atom>> {
        unsafe {
            let mut targets = ptr::null_mut();
            let mut n_targets = mem::uninitialized();
            let ret = from_glib(ffi::gtk_clipboard_wait_for_targets(self.as_ref().to_glib_none().0, &mut targets, &mut n_targets));
            if ret { Some(FromGlibContainer::from_glib_container_num(targets, n_targets as usize)) } else { None }
        }
    }

    fn wait_for_text(&self) -> Option<GString> {
        unsafe {
            from_glib_full(ffi::gtk_clipboard_wait_for_text(self.as_ref().to_glib_none().0))
        }
    }

    fn wait_for_uris(&self) -> Vec<GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gtk_clipboard_wait_for_uris(self.as_ref().to_glib_none().0))
        }
    }

    fn wait_is_image_available(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_clipboard_wait_is_image_available(self.as_ref().to_glib_none().0))
        }
    }

    fn wait_is_rich_text_available<P: IsA<TextBuffer>>(&self, buffer: &P) -> bool {
        unsafe {
            from_glib(ffi::gtk_clipboard_wait_is_rich_text_available(self.as_ref().to_glib_none().0, buffer.as_ref().to_glib_none().0))
        }
    }

    fn wait_is_target_available(&self, target: &gdk::Atom) -> bool {
        unsafe {
            from_glib(ffi::gtk_clipboard_wait_is_target_available(self.as_ref().to_glib_none().0, target.to_glib_none().0))
        }
    }

    fn wait_is_text_available(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_clipboard_wait_is_text_available(self.as_ref().to_glib_none().0))
        }
    }

    fn wait_is_uris_available(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_clipboard_wait_is_uris_available(self.as_ref().to_glib_none().0))
        }
    }

    //fn connect_owner_change<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Ignored event: Gdk.EventOwnerChange
    //}
}

impl fmt::Display for Clipboard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Clipboard")
    }
}
