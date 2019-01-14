// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use TreeModel;
use TreePath;
use ffi;
use glib;
use glib::object::IsA;
use glib::translate::*;

glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct TreeRowReference(Boxed<ffi::GtkTreeRowReference>);

    match fn {
        copy => |ptr| ffi::gtk_tree_row_reference_copy(mut_override(ptr)),
        free => |ptr| ffi::gtk_tree_row_reference_free(ptr),
        get_type => || ffi::gtk_tree_row_reference_get_type(),
    }
}

impl TreeRowReference {
    pub fn new<P: IsA<TreeModel>>(model: &P, path: &TreePath) -> Option<TreeRowReference> {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gtk_tree_row_reference_new(model.as_ref().to_glib_none().0, mut_override(path.to_glib_none().0)))
        }
    }

    pub fn new_proxy<P: IsA<glib::Object>, Q: IsA<TreeModel>>(proxy: &P, model: &Q, path: &TreePath) -> Option<TreeRowReference> {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gtk_tree_row_reference_new_proxy(proxy.as_ref().to_glib_none().0, model.as_ref().to_glib_none().0, mut_override(path.to_glib_none().0)))
        }
    }

    pub fn get_model(&self) -> TreeModel {
        unsafe {
            from_glib_none(ffi::gtk_tree_row_reference_get_model(mut_override(self.to_glib_none().0)))
        }
    }

    pub fn get_path(&self) -> Option<TreePath> {
        unsafe {
            from_glib_full(ffi::gtk_tree_row_reference_get_path(mut_override(self.to_glib_none().0)))
        }
    }

    pub fn valid(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_row_reference_valid(mut_override(self.to_glib_none().0)))
        }
    }

    pub fn deleted<P: IsA<glib::Object>>(proxy: &P, path: &TreePath) {
        assert_initialized_main_thread!();
        unsafe {
            ffi::gtk_tree_row_reference_deleted(proxy.as_ref().to_glib_none().0, mut_override(path.to_glib_none().0));
        }
    }

    pub fn inserted<P: IsA<glib::Object>>(proxy: &P, path: &TreePath) {
        assert_initialized_main_thread!();
        unsafe {
            ffi::gtk_tree_row_reference_inserted(proxy.as_ref().to_glib_none().0, mut_override(path.to_glib_none().0));
        }
    }

    //pub fn reordered<P: IsA<glib::Object>>(proxy: &P, path: &mut TreePath, iter: &mut TreeIter, new_order: &[i32]) {
    //    unsafe { TODO: call ffi::gtk_tree_row_reference_reordered() }
    //}
}
