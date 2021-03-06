// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use CellRenderer;
use TreePath;
use ffi;
use glib::StaticType;
use glib::Value;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct CellRendererToggle(Object<ffi::GtkCellRendererToggle, ffi::GtkCellRendererToggleClass, CellRendererToggleClass>) @extends CellRenderer;

    match fn {
        get_type => || ffi::gtk_cell_renderer_toggle_get_type(),
    }
}

impl CellRendererToggle {
    pub fn new() -> CellRendererToggle {
        assert_initialized_main_thread!();
        unsafe {
            CellRenderer::from_glib_none(ffi::gtk_cell_renderer_toggle_new()).unsafe_cast()
        }
    }
}

impl Default for CellRendererToggle {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_CELL_RENDERER_TOGGLE: Option<&CellRendererToggle> = None;

pub trait CellRendererToggleExt: 'static {
    fn get_activatable(&self) -> bool;

    fn get_active(&self) -> bool;

    fn get_radio(&self) -> bool;

    fn set_activatable(&self, setting: bool);

    fn set_active(&self, setting: bool);

    fn set_radio(&self, radio: bool);

    fn get_property_inconsistent(&self) -> bool;

    fn set_property_inconsistent(&self, inconsistent: bool);

    fn get_property_indicator_size(&self) -> i32;

    fn set_property_indicator_size(&self, indicator_size: i32);

    fn connect_toggled<F: Fn(&Self, TreePath) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_activatable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_active_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_inconsistent_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_indicator_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_radio_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<CellRendererToggle>> CellRendererToggleExt for O {
    fn get_activatable(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_cell_renderer_toggle_get_activatable(self.as_ref().to_glib_none().0))
        }
    }

    fn get_active(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_cell_renderer_toggle_get_active(self.as_ref().to_glib_none().0))
        }
    }

    fn get_radio(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_cell_renderer_toggle_get_radio(self.as_ref().to_glib_none().0))
        }
    }

    fn set_activatable(&self, setting: bool) {
        unsafe {
            ffi::gtk_cell_renderer_toggle_set_activatable(self.as_ref().to_glib_none().0, setting.to_glib());
        }
    }

    fn set_active(&self, setting: bool) {
        unsafe {
            ffi::gtk_cell_renderer_toggle_set_active(self.as_ref().to_glib_none().0, setting.to_glib());
        }
    }

    fn set_radio(&self, radio: bool) {
        unsafe {
            ffi::gtk_cell_renderer_toggle_set_radio(self.as_ref().to_glib_none().0, radio.to_glib());
        }
    }

    fn get_property_inconsistent(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"inconsistent\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_inconsistent(&self, inconsistent: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"inconsistent\0".as_ptr() as *const _, Value::from(&inconsistent).to_glib_none().0);
        }
    }

    fn get_property_indicator_size(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"indicator-size\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_indicator_size(&self, indicator_size: i32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"indicator-size\0".as_ptr() as *const _, Value::from(&indicator_size).to_glib_none().0);
        }
    }

    fn connect_toggled<F: Fn(&Self, TreePath) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"toggled\0".as_ptr() as *const _,
                Some(transmute(toggled_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_activatable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::activatable\0".as_ptr() as *const _,
                Some(transmute(notify_activatable_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_active_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::active\0".as_ptr() as *const _,
                Some(transmute(notify_active_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_inconsistent_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::inconsistent\0".as_ptr() as *const _,
                Some(transmute(notify_inconsistent_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_indicator_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::indicator-size\0".as_ptr() as *const _,
                Some(transmute(notify_indicator_size_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_radio_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::radio\0".as_ptr() as *const _,
                Some(transmute(notify_radio_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn toggled_trampoline<P, F: Fn(&P, TreePath) + 'static>(this: *mut ffi::GtkCellRendererToggle, path: *mut libc::c_char, f: glib_ffi::gpointer)
where P: IsA<CellRendererToggle> {
    let f: &F = &*(f as *const F);
    let path = from_glib_full(ffi::gtk_tree_path_new_from_string(path));
    f(&CellRendererToggle::from_glib_borrow(this).unsafe_cast(), path)
}

unsafe extern "C" fn notify_activatable_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkCellRendererToggle, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CellRendererToggle> {
    let f: &F = &*(f as *const F);
    f(&CellRendererToggle::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_active_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkCellRendererToggle, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CellRendererToggle> {
    let f: &F = &*(f as *const F);
    f(&CellRendererToggle::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_inconsistent_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkCellRendererToggle, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CellRendererToggle> {
    let f: &F = &*(f as *const F);
    f(&CellRendererToggle::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_indicator_size_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkCellRendererToggle, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CellRendererToggle> {
    let f: &F = &*(f as *const F);
    f(&CellRendererToggle::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_radio_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkCellRendererToggle, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CellRendererToggle> {
    let f: &F = &*(f as *const F);
    f(&CellRendererToggle::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for CellRendererToggle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CellRendererToggle")
    }
}
