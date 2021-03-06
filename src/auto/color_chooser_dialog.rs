// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Bin;
use Buildable;
use ColorChooser;
use Container;
use Dialog;
use Widget;
use Window;
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
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct ColorChooserDialog(Object<ffi::GtkColorChooserDialog, ffi::GtkColorChooserDialogClass, ColorChooserDialogClass>) @extends Dialog, Window, Bin, Container, Widget, @implements Buildable, ColorChooser;

    match fn {
        get_type => || ffi::gtk_color_chooser_dialog_get_type(),
    }
}

impl ColorChooserDialog {
    pub fn new<P: IsA<Window>>(title: Option<&str>, parent: Option<&P>) -> ColorChooserDialog {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_color_chooser_dialog_new(title.to_glib_none().0, parent.map(|p| p.as_ref()).to_glib_none().0)).unsafe_cast()
        }
    }
}

pub const NONE_COLOR_CHOOSER_DIALOG: Option<&ColorChooserDialog> = None;

pub trait ColorChooserDialogExt: 'static {
    fn get_property_show_editor(&self) -> bool;

    fn set_property_show_editor(&self, show_editor: bool);

    fn connect_property_show_editor_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<ColorChooserDialog>> ColorChooserDialogExt for O {
    fn get_property_show_editor(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"show-editor\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_show_editor(&self, show_editor: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"show-editor\0".as_ptr() as *const _, Value::from(&show_editor).to_glib_none().0);
        }
    }

    fn connect_property_show_editor_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::show-editor\0".as_ptr() as *const _,
                Some(transmute(notify_show_editor_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn notify_show_editor_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkColorChooserDialog, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ColorChooserDialog> {
    let f: &F = &*(f as *const F);
    f(&ColorChooserDialog::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for ColorChooserDialog {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ColorChooserDialog")
    }
}
