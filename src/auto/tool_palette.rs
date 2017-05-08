// This file was generated by gir (f493ea3) from gir-files (71d73f0)
// DO NOT EDIT

use Container;
use DestDefaults;
use Orientable;
use Scrollable;
use SelectionData;
use TargetEntry;
use ToolItem;
use ToolItemGroup;
use ToolPaletteDragTargets;
use ToolbarStyle;
use Widget;
use ffi;
use gdk;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;
use gobject_ffi;
use std::mem::transmute;

glib_wrapper! {
    pub struct ToolPalette(Object<ffi::GtkToolPalette>): Container, Widget, Orientable, Scrollable;

    match fn {
        get_type => || ffi::gtk_tool_palette_get_type(),
    }
}

impl ToolPalette {
    pub fn new() -> ToolPalette {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_tool_palette_new()).downcast_unchecked()
        }
    }

    pub fn add_drag_dest<P: IsA<Widget>>(&self, widget: &P, flags: DestDefaults, targets: ToolPaletteDragTargets, actions: gdk::DragAction) {
        unsafe {
            ffi::gtk_tool_palette_add_drag_dest(self.to_glib_none().0, widget.to_glib_none().0, flags.to_glib(), targets.to_glib(), actions.to_glib());
        }
    }

    pub fn get_drag_item(&self, selection: &SelectionData) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_tool_palette_get_drag_item(self.to_glib_none().0, selection.to_glib_none().0))
        }
    }

    pub fn get_drop_group(&self, x: i32, y: i32) -> Option<ToolItemGroup> {
        unsafe {
            from_glib_none(ffi::gtk_tool_palette_get_drop_group(self.to_glib_none().0, x, y))
        }
    }

    pub fn get_drop_item(&self, x: i32, y: i32) -> Option<ToolItem> {
        unsafe {
            from_glib_none(ffi::gtk_tool_palette_get_drop_item(self.to_glib_none().0, x, y))
        }
    }

    pub fn get_exclusive(&self, group: &ToolItemGroup) -> bool {
        unsafe {
            from_glib(ffi::gtk_tool_palette_get_exclusive(self.to_glib_none().0, group.to_glib_none().0))
        }
    }

    pub fn get_expand(&self, group: &ToolItemGroup) -> bool {
        unsafe {
            from_glib(ffi::gtk_tool_palette_get_expand(self.to_glib_none().0, group.to_glib_none().0))
        }
    }

    pub fn get_group_position(&self, group: &ToolItemGroup) -> i32 {
        unsafe {
            ffi::gtk_tool_palette_get_group_position(self.to_glib_none().0, group.to_glib_none().0)
        }
    }

    pub fn get_icon_size(&self) -> i32 {
        unsafe {
            ffi::gtk_tool_palette_get_icon_size(self.to_glib_none().0)
        }
    }

    pub fn get_style(&self) -> ToolbarStyle {
        unsafe {
            from_glib(ffi::gtk_tool_palette_get_style(self.to_glib_none().0))
        }
    }

    pub fn set_drag_source(&self, targets: ToolPaletteDragTargets) {
        unsafe {
            ffi::gtk_tool_palette_set_drag_source(self.to_glib_none().0, targets.to_glib());
        }
    }

    pub fn set_exclusive(&self, group: &ToolItemGroup, exclusive: bool) {
        unsafe {
            ffi::gtk_tool_palette_set_exclusive(self.to_glib_none().0, group.to_glib_none().0, exclusive.to_glib());
        }
    }

    pub fn set_expand(&self, group: &ToolItemGroup, expand: bool) {
        unsafe {
            ffi::gtk_tool_palette_set_expand(self.to_glib_none().0, group.to_glib_none().0, expand.to_glib());
        }
    }

    pub fn set_group_position(&self, group: &ToolItemGroup, position: i32) {
        unsafe {
            ffi::gtk_tool_palette_set_group_position(self.to_glib_none().0, group.to_glib_none().0, position);
        }
    }

    pub fn set_icon_size(&self, icon_size: i32) {
        unsafe {
            ffi::gtk_tool_palette_set_icon_size(self.to_glib_none().0, icon_size);
        }
    }

    pub fn set_style(&self, style: ToolbarStyle) {
        unsafe {
            ffi::gtk_tool_palette_set_style(self.to_glib_none().0, style.to_glib());
        }
    }

    pub fn unset_icon_size(&self) {
        unsafe {
            ffi::gtk_tool_palette_unset_icon_size(self.to_glib_none().0);
        }
    }

    pub fn unset_style(&self) {
        unsafe {
            ffi::gtk_tool_palette_unset_style(self.to_glib_none().0);
        }
    }

    pub fn get_property_icon_size_set(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "icon-size-set".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    pub fn set_property_icon_size_set(&self, icon_size_set: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "icon-size-set".to_glib_none().0, Value::from(&icon_size_set).to_glib_none().0);
        }
    }

    pub fn get_property_toolbar_style(&self) -> ToolbarStyle {
        let mut value = Value::from(&0);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "toolbar-style".to_glib_none().0, value.to_glib_none_mut().0);
            from_glib(transmute(value.get::<i32>().unwrap()))
        }
    }

    pub fn set_property_toolbar_style(&self, toolbar_style: ToolbarStyle) {
        let toolbar_style = toolbar_style.to_glib() as i32;
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "toolbar-style".to_glib_none().0, Value::from(&toolbar_style).to_glib_none().0);
        }
    }

    pub fn get_drag_target_group() -> Option<TargetEntry> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gtk_tool_palette_get_drag_target_group())
        }
    }

    pub fn get_drag_target_item() -> Option<TargetEntry> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gtk_tool_palette_get_drag_target_item())
        }
    }
}
