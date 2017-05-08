// This file was generated by gir (f493ea3) from gir-files (71d73f0)
// DO NOT EDIT

use Object;
use ffi;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use libc;
use pango;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct FontChooser(Object<ffi::GtkFontChooser>);

    match fn {
        get_type => || ffi::gtk_font_chooser_get_type(),
    }
}

pub trait FontChooserExt {
    fn get_font(&self) -> Option<String>;

    fn get_font_desc(&self) -> Option<pango::FontDescription>;

    fn get_font_face(&self) -> Option<pango::FontFace>;

    fn get_font_family(&self) -> Option<pango::FontFamily>;

    #[cfg(feature = "v3_18")]
    fn get_font_map(&self) -> Option<pango::FontMap>;

    fn get_font_size(&self) -> i32;

    fn get_preview_text(&self) -> Option<String>;

    fn get_show_preview_entry(&self) -> bool;

    //fn set_filter_func<'a, P: Into<Option<&'a /*Unimplemented*/FontFilterFunc>>, Q: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, filter: P, user_data: Q, destroy: /*Unknown conversion*//*Unimplemented*/DestroyNotify);

    fn set_font(&self, fontname: &str);

    fn set_font_desc(&self, font_desc: &pango::FontDescription);

    #[cfg(feature = "v3_18")]
    fn set_font_map<'a, P: Into<Option<&'a pango::FontMap>>>(&self, fontmap: P);

    fn set_preview_text(&self, text: &str);

    fn set_show_preview_entry(&self, show_preview_entry: bool);

    fn connect_font_activated<F: Fn(&Self, &str) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<FontChooser> + IsA<Object>> FontChooserExt for O {
    fn get_font(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gtk_font_chooser_get_font(self.to_glib_none().0))
        }
    }

    fn get_font_desc(&self) -> Option<pango::FontDescription> {
        unsafe {
            from_glib_full(ffi::gtk_font_chooser_get_font_desc(self.to_glib_none().0))
        }
    }

    fn get_font_face(&self) -> Option<pango::FontFace> {
        unsafe {
            from_glib_none(ffi::gtk_font_chooser_get_font_face(self.to_glib_none().0))
        }
    }

    fn get_font_family(&self) -> Option<pango::FontFamily> {
        unsafe {
            from_glib_none(ffi::gtk_font_chooser_get_font_family(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_18")]
    fn get_font_map(&self) -> Option<pango::FontMap> {
        unsafe {
            from_glib_full(ffi::gtk_font_chooser_get_font_map(self.to_glib_none().0))
        }
    }

    fn get_font_size(&self) -> i32 {
        unsafe {
            ffi::gtk_font_chooser_get_font_size(self.to_glib_none().0)
        }
    }

    fn get_preview_text(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gtk_font_chooser_get_preview_text(self.to_glib_none().0))
        }
    }

    fn get_show_preview_entry(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_font_chooser_get_show_preview_entry(self.to_glib_none().0))
        }
    }

    //fn set_filter_func<'a, P: Into<Option<&'a /*Unimplemented*/FontFilterFunc>>, Q: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, filter: P, user_data: Q, destroy: /*Unknown conversion*//*Unimplemented*/DestroyNotify) {
    //    unsafe { TODO: call ffi::gtk_font_chooser_set_filter_func() }
    //}

    fn set_font(&self, fontname: &str) {
        unsafe {
            ffi::gtk_font_chooser_set_font(self.to_glib_none().0, fontname.to_glib_none().0);
        }
    }

    fn set_font_desc(&self, font_desc: &pango::FontDescription) {
        unsafe {
            ffi::gtk_font_chooser_set_font_desc(self.to_glib_none().0, font_desc.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_18")]
    fn set_font_map<'a, P: Into<Option<&'a pango::FontMap>>>(&self, fontmap: P) {
        let fontmap = fontmap.into();
        let fontmap = fontmap.to_glib_none();
        unsafe {
            ffi::gtk_font_chooser_set_font_map(self.to_glib_none().0, fontmap.0);
        }
    }

    fn set_preview_text(&self, text: &str) {
        unsafe {
            ffi::gtk_font_chooser_set_preview_text(self.to_glib_none().0, text.to_glib_none().0);
        }
    }

    fn set_show_preview_entry(&self, show_preview_entry: bool) {
        unsafe {
            ffi::gtk_font_chooser_set_show_preview_entry(self.to_glib_none().0, show_preview_entry.to_glib());
        }
    }

    fn connect_font_activated<F: Fn(&Self, &str) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &str) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "font-activated",
                transmute(font_activated_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn font_activated_trampoline<P>(this: *mut ffi::GtkFontChooser, fontname: *mut libc::c_char, f: glib_ffi::gpointer)
where P: IsA<FontChooser> {
    callback_guard!();
    let f: &Box_<Fn(&P, &str) + 'static> = transmute(f);
    f(&FontChooser::from_glib_none(this).downcast_unchecked(), &String::from_glib_none(fontname))
}
