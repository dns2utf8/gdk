// This file was generated by gir (6a48033) from gir-files (1069259)
// DO NOT EDIT

#[cfg(feature = "v3_22")]
use Window;
#[cfg(feature = "v3_22")]
use cairo;
use ffi;
use glib;
#[cfg(feature = "v3_22")]
use glib::object::Downcast;
use glib::object::IsA;
#[cfg(feature = "v3_22")]
use glib::signal::SignalHandlerId;
#[cfg(feature = "v3_22")]
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
#[cfg(feature = "v3_22")]
use std::boxed::Box as Box_;
use std::mem;
#[cfg(feature = "v3_22")]
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct DrawingContext(Object<ffi::GdkDrawingContext>);

    match fn {
        get_type => || ffi::gdk_drawing_context_get_type(),
    }
}

pub trait DrawingContextExt {
    #[cfg(feature = "v3_22")]
    fn get_cairo_context(&self) -> Option<cairo::Context>;

    #[cfg(feature = "v3_22")]
    fn get_clip(&self) -> Option<cairo::Region>;

    #[cfg(feature = "v3_22")]
    fn get_window(&self) -> Option<Window>;

    #[cfg(feature = "v3_22")]
    fn is_valid(&self) -> bool;

    #[cfg(feature = "v3_22")]
    fn connect_property_clip_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(feature = "v3_22")]
    fn connect_property_window_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DrawingContext> + IsA<glib::object::Object>> DrawingContextExt for O {
    #[cfg(feature = "v3_22")]
    fn get_cairo_context(&self) -> Option<cairo::Context> {
        unsafe {
            from_glib_none(ffi::gdk_drawing_context_get_cairo_context(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_22")]
    fn get_clip(&self) -> Option<cairo::Region> {
        unsafe {
            from_glib_full(ffi::gdk_drawing_context_get_clip(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_22")]
    fn get_window(&self) -> Option<Window> {
        unsafe {
            from_glib_none(ffi::gdk_drawing_context_get_window(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_22")]
    fn is_valid(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_drawing_context_is_valid(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_22")]
    fn connect_property_clip_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::clip",
                transmute(notify_clip_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(feature = "v3_22")]
    fn connect_property_window_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::window",
                transmute(notify_window_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(feature = "v3_22")]
unsafe extern "C" fn notify_clip_trampoline<P>(this: *mut ffi::GdkDrawingContext, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DrawingContext> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DrawingContext::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(feature = "v3_22")]
unsafe extern "C" fn notify_window_trampoline<P>(this: *mut ffi::GdkDrawingContext, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DrawingContext> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DrawingContext::from_glib_borrow(this).downcast_unchecked())
}
