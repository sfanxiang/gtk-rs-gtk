// Copyright 2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use glib::translate::*;
use glib::object::IsA;
use FlowBox;

use std::ptr;

pub trait FlowBoxExtManual: 'static {
    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn unbind_model(&self);
}

impl<O: IsA<FlowBox>> FlowBoxExtManual for O {
    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn unbind_model(&self) {
        unsafe {
            ffi::gtk_flow_box_bind_model(
                self.as_ref().to_glib_none().0,
                ptr::null_mut(),
                None,
                ptr::null_mut(),
                None,
            )
        }
    }
}
