// This file was generated by gir (https://github.com/gtk-rs/gir @ 6855214)
// from gir-files (https://github.com/gtk-rs/gir-files @ cf55bdc)
// DO NOT EDIT

use Adjustment;
use Bin;
use Buildable;
use Container;
use CornerType;
use DirectionType;
use PolicyType;
#[cfg(any(feature = "v3_16", feature = "dox"))]
use PositionType;
use ScrollType;
use ShadowType;
use Widget;
use ffi;
use glib;
use glib::StaticType;
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
    pub struct ScrolledWindow(Object<ffi::GtkScrolledWindow, ffi::GtkScrolledWindowClass>): Bin, Container, Widget, Buildable;

    match fn {
        get_type => || ffi::gtk_scrolled_window_get_type(),
    }
}

impl ScrolledWindow {
    pub fn new<'a, 'b, P: Into<Option<&'a Adjustment>>, Q: Into<Option<&'b Adjustment>>>(hadjustment: P, vadjustment: Q) -> ScrolledWindow {
        assert_initialized_main_thread!();
        let hadjustment = hadjustment.into();
        let hadjustment = hadjustment.to_glib_none();
        let vadjustment = vadjustment.into();
        let vadjustment = vadjustment.to_glib_none();
        unsafe {
            Widget::from_glib_none(ffi::gtk_scrolled_window_new(hadjustment.0, vadjustment.0)).downcast_unchecked()
        }
    }
}

pub trait ScrolledWindowExt {
    #[cfg_attr(feature = "v3_8", deprecated)]
    fn add_with_viewport<P: IsA<Widget>>(&self, child: &P);

    fn get_capture_button_press(&self) -> bool;

    fn get_hadjustment(&self) -> Option<Adjustment>;

    fn get_hscrollbar(&self) -> Option<Widget>;

    fn get_kinetic_scrolling(&self) -> bool;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_max_content_height(&self) -> i32;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_max_content_width(&self) -> i32;

    fn get_min_content_height(&self) -> i32;

    fn get_min_content_width(&self) -> i32;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn get_overlay_scrolling(&self) -> bool;

    fn get_placement(&self) -> CornerType;

    fn get_policy(&self) -> (PolicyType, PolicyType);

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_propagate_natural_height(&self) -> bool;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_propagate_natural_width(&self) -> bool;

    fn get_shadow_type(&self) -> ShadowType;

    fn get_vadjustment(&self) -> Option<Adjustment>;

    fn get_vscrollbar(&self) -> Option<Widget>;

    fn set_capture_button_press(&self, capture_button_press: bool);

    fn set_hadjustment(&self, hadjustment: &Adjustment);

    fn set_kinetic_scrolling(&self, kinetic_scrolling: bool);

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn set_max_content_height(&self, height: i32);

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn set_max_content_width(&self, width: i32);

    fn set_min_content_height(&self, height: i32);

    fn set_min_content_width(&self, width: i32);

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn set_overlay_scrolling(&self, overlay_scrolling: bool);

    fn set_placement(&self, window_placement: CornerType);

    fn set_policy(&self, hscrollbar_policy: PolicyType, vscrollbar_policy: PolicyType);

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn set_propagate_natural_height(&self, propagate: bool);

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn set_propagate_natural_width(&self, propagate: bool);

    fn set_shadow_type(&self, type_: ShadowType);

    fn set_vadjustment(&self, vadjustment: &Adjustment);

    fn unset_placement(&self);

    fn get_property_hscrollbar_policy(&self) -> PolicyType;

    fn set_property_hscrollbar_policy(&self, hscrollbar_policy: PolicyType);

    fn get_property_vscrollbar_policy(&self) -> PolicyType;

    fn set_property_vscrollbar_policy(&self, vscrollbar_policy: PolicyType);

    fn get_property_window_placement(&self) -> CornerType;

    fn set_property_window_placement(&self, window_placement: CornerType);

    #[cfg_attr(feature = "v3_10", deprecated)]
    fn get_property_window_placement_set(&self) -> bool;

    #[cfg_attr(feature = "v3_10", deprecated)]
    fn set_property_window_placement_set(&self, window_placement_set: bool);

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_edge_overshot<F: Fn(&Self, PositionType) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_edge_reached<F: Fn(&Self, PositionType) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_move_focus_out<F: Fn(&Self, DirectionType) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_move_focus_out(&self, direction_type: DirectionType);

    fn connect_scroll_child<F: Fn(&Self, ScrollType, bool) -> bool + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_scroll_child(&self, scroll: ScrollType, horizontal: bool) -> bool;

    fn connect_property_hadjustment_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_hscrollbar_policy_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_kinetic_scrolling_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn connect_property_max_content_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn connect_property_max_content_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_min_content_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_min_content_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_property_overlay_scrolling_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn connect_property_propagate_natural_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn connect_property_propagate_natural_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_shadow_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_vadjustment_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_vscrollbar_policy_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_window_placement_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg_attr(feature = "v3_10", deprecated)]
    fn connect_property_window_placement_set_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<ScrolledWindow> + IsA<glib::object::Object> + glib::object::ObjectExt> ScrolledWindowExt for O {
    fn add_with_viewport<P: IsA<Widget>>(&self, child: &P) {
        unsafe {
            ffi::gtk_scrolled_window_add_with_viewport(self.to_glib_none().0, child.to_glib_none().0);
        }
    }

    fn get_capture_button_press(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_scrolled_window_get_capture_button_press(self.to_glib_none().0))
        }
    }

    fn get_hadjustment(&self) -> Option<Adjustment> {
        unsafe {
            from_glib_none(ffi::gtk_scrolled_window_get_hadjustment(self.to_glib_none().0))
        }
    }

    fn get_hscrollbar(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_scrolled_window_get_hscrollbar(self.to_glib_none().0))
        }
    }

    fn get_kinetic_scrolling(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_scrolled_window_get_kinetic_scrolling(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_max_content_height(&self) -> i32 {
        unsafe {
            ffi::gtk_scrolled_window_get_max_content_height(self.to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_max_content_width(&self) -> i32 {
        unsafe {
            ffi::gtk_scrolled_window_get_max_content_width(self.to_glib_none().0)
        }
    }

    fn get_min_content_height(&self) -> i32 {
        unsafe {
            ffi::gtk_scrolled_window_get_min_content_height(self.to_glib_none().0)
        }
    }

    fn get_min_content_width(&self) -> i32 {
        unsafe {
            ffi::gtk_scrolled_window_get_min_content_width(self.to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn get_overlay_scrolling(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_scrolled_window_get_overlay_scrolling(self.to_glib_none().0))
        }
    }

    fn get_placement(&self) -> CornerType {
        unsafe {
            from_glib(ffi::gtk_scrolled_window_get_placement(self.to_glib_none().0))
        }
    }

    fn get_policy(&self) -> (PolicyType, PolicyType) {
        unsafe {
            let mut hscrollbar_policy = mem::uninitialized();
            let mut vscrollbar_policy = mem::uninitialized();
            ffi::gtk_scrolled_window_get_policy(self.to_glib_none().0, &mut hscrollbar_policy, &mut vscrollbar_policy);
            (from_glib(hscrollbar_policy), from_glib(vscrollbar_policy))
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_propagate_natural_height(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_scrolled_window_get_propagate_natural_height(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_propagate_natural_width(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_scrolled_window_get_propagate_natural_width(self.to_glib_none().0))
        }
    }

    fn get_shadow_type(&self) -> ShadowType {
        unsafe {
            from_glib(ffi::gtk_scrolled_window_get_shadow_type(self.to_glib_none().0))
        }
    }

    fn get_vadjustment(&self) -> Option<Adjustment> {
        unsafe {
            from_glib_none(ffi::gtk_scrolled_window_get_vadjustment(self.to_glib_none().0))
        }
    }

    fn get_vscrollbar(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_scrolled_window_get_vscrollbar(self.to_glib_none().0))
        }
    }

    fn set_capture_button_press(&self, capture_button_press: bool) {
        unsafe {
            ffi::gtk_scrolled_window_set_capture_button_press(self.to_glib_none().0, capture_button_press.to_glib());
        }
    }

    fn set_hadjustment(&self, hadjustment: &Adjustment) {
        unsafe {
            ffi::gtk_scrolled_window_set_hadjustment(self.to_glib_none().0, hadjustment.to_glib_none().0);
        }
    }

    fn set_kinetic_scrolling(&self, kinetic_scrolling: bool) {
        unsafe {
            ffi::gtk_scrolled_window_set_kinetic_scrolling(self.to_glib_none().0, kinetic_scrolling.to_glib());
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn set_max_content_height(&self, height: i32) {
        unsafe {
            ffi::gtk_scrolled_window_set_max_content_height(self.to_glib_none().0, height);
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn set_max_content_width(&self, width: i32) {
        unsafe {
            ffi::gtk_scrolled_window_set_max_content_width(self.to_glib_none().0, width);
        }
    }

    fn set_min_content_height(&self, height: i32) {
        unsafe {
            ffi::gtk_scrolled_window_set_min_content_height(self.to_glib_none().0, height);
        }
    }

    fn set_min_content_width(&self, width: i32) {
        unsafe {
            ffi::gtk_scrolled_window_set_min_content_width(self.to_glib_none().0, width);
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn set_overlay_scrolling(&self, overlay_scrolling: bool) {
        unsafe {
            ffi::gtk_scrolled_window_set_overlay_scrolling(self.to_glib_none().0, overlay_scrolling.to_glib());
        }
    }

    fn set_placement(&self, window_placement: CornerType) {
        unsafe {
            ffi::gtk_scrolled_window_set_placement(self.to_glib_none().0, window_placement.to_glib());
        }
    }

    fn set_policy(&self, hscrollbar_policy: PolicyType, vscrollbar_policy: PolicyType) {
        unsafe {
            ffi::gtk_scrolled_window_set_policy(self.to_glib_none().0, hscrollbar_policy.to_glib(), vscrollbar_policy.to_glib());
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn set_propagate_natural_height(&self, propagate: bool) {
        unsafe {
            ffi::gtk_scrolled_window_set_propagate_natural_height(self.to_glib_none().0, propagate.to_glib());
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn set_propagate_natural_width(&self, propagate: bool) {
        unsafe {
            ffi::gtk_scrolled_window_set_propagate_natural_width(self.to_glib_none().0, propagate.to_glib());
        }
    }

    fn set_shadow_type(&self, type_: ShadowType) {
        unsafe {
            ffi::gtk_scrolled_window_set_shadow_type(self.to_glib_none().0, type_.to_glib());
        }
    }

    fn set_vadjustment(&self, vadjustment: &Adjustment) {
        unsafe {
            ffi::gtk_scrolled_window_set_vadjustment(self.to_glib_none().0, vadjustment.to_glib_none().0);
        }
    }

    fn unset_placement(&self) {
        unsafe {
            ffi::gtk_scrolled_window_unset_placement(self.to_glib_none().0);
        }
    }

    fn get_property_hscrollbar_policy(&self) -> PolicyType {
        unsafe {
            let mut value = Value::from_type(<PolicyType as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "hscrollbar-policy".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_hscrollbar_policy(&self, hscrollbar_policy: PolicyType) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "hscrollbar-policy".to_glib_none().0, Value::from(&hscrollbar_policy).to_glib_none().0);
        }
    }

    fn get_property_vscrollbar_policy(&self) -> PolicyType {
        unsafe {
            let mut value = Value::from_type(<PolicyType as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "vscrollbar-policy".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_vscrollbar_policy(&self, vscrollbar_policy: PolicyType) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "vscrollbar-policy".to_glib_none().0, Value::from(&vscrollbar_policy).to_glib_none().0);
        }
    }

    fn get_property_window_placement(&self) -> CornerType {
        unsafe {
            let mut value = Value::from_type(<CornerType as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "window-placement".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_window_placement(&self, window_placement: CornerType) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "window-placement".to_glib_none().0, Value::from(&window_placement).to_glib_none().0);
        }
    }

    fn get_property_window_placement_set(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "window-placement-set".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_window_placement_set(&self, window_placement_set: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "window-placement-set".to_glib_none().0, Value::from(&window_placement_set).to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_edge_overshot<F: Fn(&Self, PositionType) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, PositionType) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "edge-overshot",
                transmute(edge_overshot_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_edge_reached<F: Fn(&Self, PositionType) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, PositionType) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "edge-reached",
                transmute(edge_reached_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_move_focus_out<F: Fn(&Self, DirectionType) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, DirectionType) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "move-focus-out",
                transmute(move_focus_out_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn emit_move_focus_out(&self, direction_type: DirectionType) {
        let _ = self.emit("move-focus-out", &[&direction_type]).unwrap();
    }

    fn connect_scroll_child<F: Fn(&Self, ScrollType, bool) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, ScrollType, bool) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "scroll-child",
                transmute(scroll_child_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn emit_scroll_child(&self, scroll: ScrollType, horizontal: bool) -> bool {
        let res = self.emit("scroll-child", &[&scroll, &horizontal]).unwrap();
        res.unwrap().get().unwrap()
    }

    fn connect_property_hadjustment_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::hadjustment",
                transmute(notify_hadjustment_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_hscrollbar_policy_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::hscrollbar-policy",
                transmute(notify_hscrollbar_policy_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_kinetic_scrolling_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::kinetic-scrolling",
                transmute(notify_kinetic_scrolling_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn connect_property_max_content_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::max-content-height",
                transmute(notify_max_content_height_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn connect_property_max_content_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::max-content-width",
                transmute(notify_max_content_width_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_min_content_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::min-content-height",
                transmute(notify_min_content_height_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_min_content_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::min-content-width",
                transmute(notify_min_content_width_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_property_overlay_scrolling_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::overlay-scrolling",
                transmute(notify_overlay_scrolling_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn connect_property_propagate_natural_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::propagate-natural-height",
                transmute(notify_propagate_natural_height_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn connect_property_propagate_natural_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::propagate-natural-width",
                transmute(notify_propagate_natural_width_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_shadow_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::shadow-type",
                transmute(notify_shadow_type_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_vadjustment_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::vadjustment",
                transmute(notify_vadjustment_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_vscrollbar_policy_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::vscrollbar-policy",
                transmute(notify_vscrollbar_policy_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_window_placement_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::window-placement",
                transmute(notify_window_placement_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_window_placement_set_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::window-placement-set",
                transmute(notify_window_placement_set_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(any(feature = "v3_16", feature = "dox"))]
unsafe extern "C" fn edge_overshot_trampoline<P>(this: *mut ffi::GtkScrolledWindow, pos: ffi::GtkPositionType, f: glib_ffi::gpointer)
where P: IsA<ScrolledWindow> {
    callback_guard!();
    let f: &&(Fn(&P, PositionType) + 'static) = transmute(f);
    f(&ScrolledWindow::from_glib_borrow(this).downcast_unchecked(), from_glib(pos))
}

#[cfg(any(feature = "v3_16", feature = "dox"))]
unsafe extern "C" fn edge_reached_trampoline<P>(this: *mut ffi::GtkScrolledWindow, pos: ffi::GtkPositionType, f: glib_ffi::gpointer)
where P: IsA<ScrolledWindow> {
    callback_guard!();
    let f: &&(Fn(&P, PositionType) + 'static) = transmute(f);
    f(&ScrolledWindow::from_glib_borrow(this).downcast_unchecked(), from_glib(pos))
}

unsafe extern "C" fn move_focus_out_trampoline<P>(this: *mut ffi::GtkScrolledWindow, direction_type: ffi::GtkDirectionType, f: glib_ffi::gpointer)
where P: IsA<ScrolledWindow> {
    callback_guard!();
    let f: &&(Fn(&P, DirectionType) + 'static) = transmute(f);
    f(&ScrolledWindow::from_glib_borrow(this).downcast_unchecked(), from_glib(direction_type))
}

unsafe extern "C" fn scroll_child_trampoline<P>(this: *mut ffi::GtkScrolledWindow, scroll: ffi::GtkScrollType, horizontal: glib_ffi::gboolean, f: glib_ffi::gpointer) -> glib_ffi::gboolean
where P: IsA<ScrolledWindow> {
    callback_guard!();
    let f: &&(Fn(&P, ScrollType, bool) -> bool + 'static) = transmute(f);
    f(&ScrolledWindow::from_glib_borrow(this).downcast_unchecked(), from_glib(scroll), from_glib(horizontal)).to_glib()
}

unsafe extern "C" fn notify_hadjustment_trampoline<P>(this: *mut ffi::GtkScrolledWindow, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ScrolledWindow> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ScrolledWindow::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_hscrollbar_policy_trampoline<P>(this: *mut ffi::GtkScrolledWindow, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ScrolledWindow> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ScrolledWindow::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_kinetic_scrolling_trampoline<P>(this: *mut ffi::GtkScrolledWindow, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ScrolledWindow> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ScrolledWindow::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_22", feature = "dox"))]
unsafe extern "C" fn notify_max_content_height_trampoline<P>(this: *mut ffi::GtkScrolledWindow, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ScrolledWindow> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ScrolledWindow::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_22", feature = "dox"))]
unsafe extern "C" fn notify_max_content_width_trampoline<P>(this: *mut ffi::GtkScrolledWindow, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ScrolledWindow> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ScrolledWindow::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_min_content_height_trampoline<P>(this: *mut ffi::GtkScrolledWindow, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ScrolledWindow> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ScrolledWindow::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_min_content_width_trampoline<P>(this: *mut ffi::GtkScrolledWindow, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ScrolledWindow> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ScrolledWindow::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_16", feature = "dox"))]
unsafe extern "C" fn notify_overlay_scrolling_trampoline<P>(this: *mut ffi::GtkScrolledWindow, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ScrolledWindow> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ScrolledWindow::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_22", feature = "dox"))]
unsafe extern "C" fn notify_propagate_natural_height_trampoline<P>(this: *mut ffi::GtkScrolledWindow, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ScrolledWindow> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ScrolledWindow::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_22", feature = "dox"))]
unsafe extern "C" fn notify_propagate_natural_width_trampoline<P>(this: *mut ffi::GtkScrolledWindow, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ScrolledWindow> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ScrolledWindow::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_shadow_type_trampoline<P>(this: *mut ffi::GtkScrolledWindow, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ScrolledWindow> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ScrolledWindow::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_vadjustment_trampoline<P>(this: *mut ffi::GtkScrolledWindow, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ScrolledWindow> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ScrolledWindow::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_vscrollbar_policy_trampoline<P>(this: *mut ffi::GtkScrolledWindow, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ScrolledWindow> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ScrolledWindow::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_window_placement_trampoline<P>(this: *mut ffi::GtkScrolledWindow, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ScrolledWindow> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ScrolledWindow::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_window_placement_set_trampoline<P>(this: *mut ffi::GtkScrolledWindow, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ScrolledWindow> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ScrolledWindow::from_glib_borrow(this).downcast_unchecked())
}
