// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gdk;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::Value;
use glib_sys;
use gobject_sys;
use gtk_sys;
use std::boxed::Box as Box_;
use std::fmt;
use Buildable;
use Widget;

glib_wrapper! {
    pub struct CellEditable(Interface<gtk_sys::GtkCellEditable>) @requires Widget, Buildable;

    match fn {
        get_type => || gtk_sys::gtk_cell_editable_get_type(),
    }
}

pub const NONE_CELL_EDITABLE: Option<&CellEditable> = None;

pub trait CellEditableExt: 'static {
    fn editing_done(&self);

    fn remove_widget(&self);

    fn start_editing(&self, event: Option<&gdk::Event>);

    fn get_property_editing_canceled(&self) -> bool;

    fn set_property_editing_canceled(&self, editing_canceled: bool);

    fn connect_editing_done<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_remove_widget<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_editing_canceled_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<CellEditable>> CellEditableExt for O {
    fn editing_done(&self) {
        unsafe {
            gtk_sys::gtk_cell_editable_editing_done(self.as_ref().to_glib_none().0);
        }
    }

    fn remove_widget(&self) {
        unsafe {
            gtk_sys::gtk_cell_editable_remove_widget(self.as_ref().to_glib_none().0);
        }
    }

    fn start_editing(&self, event: Option<&gdk::Event>) {
        unsafe {
            gtk_sys::gtk_cell_editable_start_editing(
                self.as_ref().to_glib_none().0,
                mut_override(event.to_glib_none().0),
            );
        }
    }

    fn get_property_editing_canceled(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"editing-canceled\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `editing-canceled` getter")
                .unwrap()
        }
    }

    fn set_property_editing_canceled(&self, editing_canceled: bool) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"editing-canceled\0".as_ptr() as *const _,
                Value::from(&editing_canceled).to_glib_none().0,
            );
        }
    }

    fn connect_editing_done<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn editing_done_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkCellEditable,
            f: glib_sys::gpointer,
        ) where
            P: IsA<CellEditable>,
        {
            let f: &F = &*(f as *const F);
            f(&CellEditable::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"editing-done\0".as_ptr() as *const _,
                Some(*(&editing_done_trampoline::<Self, F> as *const _ as *const _)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_remove_widget<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn remove_widget_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkCellEditable,
            f: glib_sys::gpointer,
        ) where
            P: IsA<CellEditable>,
        {
            let f: &F = &*(f as *const F);
            f(&CellEditable::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"remove-widget\0".as_ptr() as *const _,
                Some(*(&remove_widget_trampoline::<Self, F> as *const _ as *const _)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_editing_canceled_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_editing_canceled_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkCellEditable,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<CellEditable>,
        {
            let f: &F = &*(f as *const F);
            f(&CellEditable::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::editing-canceled\0".as_ptr() as *const _,
                Some(*(&notify_editing_canceled_trampoline::<Self, F> as *const _ as *const _)),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for CellEditable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CellEditable")
    }
}
