// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Buildable;
use Widget;
use ffi;
use gdk;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct Invisible(Object<ffi::GtkInvisible, ffi::GtkInvisibleClass, InvisibleClass>) @extends Widget, @implements Buildable;

    match fn {
        get_type => || ffi::gtk_invisible_get_type(),
    }
}

impl Invisible {
    pub fn new() -> Invisible {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_invisible_new()).unsafe_cast()
        }
    }

    pub fn new_for_screen<P: IsA<gdk::Screen>>(screen: &P) -> Invisible {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_invisible_new_for_screen(screen.as_ref().to_glib_none().0)).unsafe_cast()
        }
    }
}

impl Default for Invisible {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_INVISIBLE: Option<&Invisible> = None;

pub trait InvisibleExt: 'static {
    fn set_screen<P: IsA<gdk::Screen>>(&self, screen: &P);

    fn connect_property_screen_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Invisible>> InvisibleExt for O {
    fn set_screen<P: IsA<gdk::Screen>>(&self, screen: &P) {
        unsafe {
            ffi::gtk_invisible_set_screen(self.as_ref().to_glib_none().0, screen.as_ref().to_glib_none().0);
        }
    }

    fn connect_property_screen_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::screen\0".as_ptr() as *const _,
                transmute(notify_screen_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_screen_trampoline<P>(this: *mut ffi::GtkInvisible, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Invisible> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Invisible::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for Invisible {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invisible")
    }
}
