// This file was generated by gir (0fe730d) from gir-files (db49619)
// DO NOT EDIT

use Bin;
use Container;
use ShadowType;
use Widget;
use ffi;
use glib;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Frame(Object<ffi::GtkFrame>): Bin, Container, Widget;

    match fn {
        get_type => || ffi::gtk_frame_get_type(),
    }
}

impl Frame {
    pub fn new<'a, P: Into<Option<&'a str>>>(label: P) -> Frame {
        assert_initialized_main_thread!();
        let label = label.into();
        let label = label.to_glib_none();
        unsafe {
            Widget::from_glib_none(ffi::gtk_frame_new(label.0)).downcast_unchecked()
        }
    }
}

pub trait FrameExt {
    fn get_label(&self) -> Option<String>;

    fn get_label_align(&self) -> (f32, f32);

    fn get_label_widget(&self) -> Option<Widget>;

    fn get_shadow_type(&self) -> ShadowType;

    fn set_label<'a, P: Into<Option<&'a str>>>(&self, label: P);

    fn set_label_align(&self, xalign: f32, yalign: f32);

    fn set_label_widget<'a, P: IsA<Widget> + 'a, Q: Into<Option<&'a P>>>(&self, label_widget: Q);

    fn set_shadow_type(&self, type_: ShadowType);

    fn get_property_label_xalign(&self) -> f32;

    fn set_property_label_xalign(&self, label_xalign: f32);

    fn get_property_label_yalign(&self) -> f32;

    fn set_property_label_yalign(&self, label_yalign: f32);

    fn connect_property_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_label_widget_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_label_xalign_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_label_yalign_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_shadow_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Frame> + IsA<glib::object::Object>> FrameExt for O {
    fn get_label(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_frame_get_label(self.to_glib_none().0))
        }
    }

    fn get_label_align(&self) -> (f32, f32) {
        unsafe {
            let mut xalign = mem::uninitialized();
            let mut yalign = mem::uninitialized();
            ffi::gtk_frame_get_label_align(self.to_glib_none().0, &mut xalign, &mut yalign);
            (xalign, yalign)
        }
    }

    fn get_label_widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_frame_get_label_widget(self.to_glib_none().0))
        }
    }

    fn get_shadow_type(&self) -> ShadowType {
        unsafe {
            from_glib(ffi::gtk_frame_get_shadow_type(self.to_glib_none().0))
        }
    }

    fn set_label<'a, P: Into<Option<&'a str>>>(&self, label: P) {
        let label = label.into();
        let label = label.to_glib_none();
        unsafe {
            ffi::gtk_frame_set_label(self.to_glib_none().0, label.0);
        }
    }

    fn set_label_align(&self, xalign: f32, yalign: f32) {
        unsafe {
            ffi::gtk_frame_set_label_align(self.to_glib_none().0, xalign, yalign);
        }
    }

    fn set_label_widget<'a, P: IsA<Widget> + 'a, Q: Into<Option<&'a P>>>(&self, label_widget: Q) {
        let label_widget = label_widget.into();
        let label_widget = label_widget.to_glib_none();
        unsafe {
            ffi::gtk_frame_set_label_widget(self.to_glib_none().0, label_widget.0);
        }
    }

    fn set_shadow_type(&self, type_: ShadowType) {
        unsafe {
            ffi::gtk_frame_set_shadow_type(self.to_glib_none().0, type_.to_glib());
        }
    }

    fn get_property_label_xalign(&self) -> f32 {
        let mut value = Value::from(&0f32);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "label-xalign".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_label_xalign(&self, label_xalign: f32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "label-xalign".to_glib_none().0, Value::from(&label_xalign).to_glib_none().0);
        }
    }

    fn get_property_label_yalign(&self) -> f32 {
        let mut value = Value::from(&0f32);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "label-yalign".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_label_yalign(&self, label_yalign: f32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "label-yalign".to_glib_none().0, Value::from(&label_yalign).to_glib_none().0);
        }
    }

    fn connect_property_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::label",
                transmute(notify_label_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_label_widget_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::label-widget",
                transmute(notify_label_widget_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_label_xalign_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::label-xalign",
                transmute(notify_label_xalign_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_label_yalign_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::label-yalign",
                transmute(notify_label_yalign_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_shadow_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::shadow-type",
                transmute(notify_shadow_type_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_label_trampoline<P>(this: *mut ffi::GtkFrame, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Frame> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Frame::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_label_widget_trampoline<P>(this: *mut ffi::GtkFrame, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Frame> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Frame::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_label_xalign_trampoline<P>(this: *mut ffi::GtkFrame, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Frame> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Frame::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_label_yalign_trampoline<P>(this: *mut ffi::GtkFrame, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Frame> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Frame::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_shadow_type_trampoline<P>(this: *mut ffi::GtkFrame, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Frame> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Frame::from_glib_borrow(this).downcast_unchecked())
}
