// This file was generated by gir (9261d77) from gir-files (469db10)
// DO NOT EDIT

use Display;
use Rectangle;
#[cfg(any(feature = "v3_22", feature = "dox"))]
use SubpixelLayout;
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
    pub struct Monitor(Object<ffi::GdkMonitor, ffi::GdkMonitorClass>);

    match fn {
        get_type => || ffi::gdk_monitor_get_type(),
    }
}

pub trait MonitorExt {
    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_display(&self) -> Option<Display>;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_geometry(&self) -> Rectangle;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_height_mm(&self) -> i32;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_manufacturer(&self) -> Option<String>;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_model(&self) -> Option<String>;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_refresh_rate(&self) -> i32;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_scale_factor(&self) -> i32;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_subpixel_layout(&self) -> SubpixelLayout;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_width_mm(&self) -> i32;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_workarea(&self) -> Rectangle;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn is_primary(&self) -> bool;

    fn get_property_display(&self) -> Option<Display>;

    fn get_property_geometry(&self) -> Option<Rectangle>;

    fn get_property_height_mm(&self) -> i32;

    fn get_property_refresh_rate(&self) -> i32;

    fn get_property_scale_factor(&self) -> i32;

    fn get_property_width_mm(&self) -> i32;

    fn get_property_workarea(&self) -> Option<Rectangle>;

    fn connect_invalidate<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_display_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_geometry_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_height_mm_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn connect_property_manufacturer_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn connect_property_model_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_refresh_rate_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_scale_factor_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn connect_property_subpixel_layout_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_width_mm_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_workarea_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Monitor> + IsA<glib::object::Object>> MonitorExt for O {
    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_display(&self) -> Option<Display> {
        unsafe {
            from_glib_none(ffi::gdk_monitor_get_display(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_geometry(&self) -> Rectangle {
        unsafe {
            let mut geometry = Rectangle::uninitialized();
            ffi::gdk_monitor_get_geometry(self.to_glib_none().0, geometry.to_glib_none_mut().0);
            geometry
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_height_mm(&self) -> i32 {
        unsafe {
            ffi::gdk_monitor_get_height_mm(self.to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_manufacturer(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gdk_monitor_get_manufacturer(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_model(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gdk_monitor_get_model(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_refresh_rate(&self) -> i32 {
        unsafe {
            ffi::gdk_monitor_get_refresh_rate(self.to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_scale_factor(&self) -> i32 {
        unsafe {
            ffi::gdk_monitor_get_scale_factor(self.to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_subpixel_layout(&self) -> SubpixelLayout {
        unsafe {
            from_glib(ffi::gdk_monitor_get_subpixel_layout(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_width_mm(&self) -> i32 {
        unsafe {
            ffi::gdk_monitor_get_width_mm(self.to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_workarea(&self) -> Rectangle {
        unsafe {
            let mut workarea = Rectangle::uninitialized();
            ffi::gdk_monitor_get_workarea(self.to_glib_none().0, workarea.to_glib_none_mut().0);
            workarea
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn is_primary(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_monitor_is_primary(self.to_glib_none().0))
        }
    }

    fn get_property_display(&self) -> Option<Display> {
        unsafe {
            let mut value = Value::from_type(<Display as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "display".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn get_property_geometry(&self) -> Option<Rectangle> {
        unsafe {
            let mut value = Value::from_type(<Rectangle as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "geometry".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn get_property_height_mm(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "height-mm".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn get_property_refresh_rate(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "refresh-rate".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn get_property_scale_factor(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "scale-factor".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn get_property_width_mm(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "width-mm".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn get_property_workarea(&self) -> Option<Rectangle> {
        unsafe {
            let mut value = Value::from_type(<Rectangle as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "workarea".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn connect_invalidate<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "invalidate",
                transmute(invalidate_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_display_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::display",
                transmute(notify_display_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_geometry_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::geometry",
                transmute(notify_geometry_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_height_mm_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::height-mm",
                transmute(notify_height_mm_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn connect_property_manufacturer_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::manufacturer",
                transmute(notify_manufacturer_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn connect_property_model_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::model",
                transmute(notify_model_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_refresh_rate_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::refresh-rate",
                transmute(notify_refresh_rate_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_scale_factor_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::scale-factor",
                transmute(notify_scale_factor_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn connect_property_subpixel_layout_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::subpixel-layout",
                transmute(notify_subpixel_layout_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_width_mm_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::width-mm",
                transmute(notify_width_mm_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_workarea_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::workarea",
                transmute(notify_workarea_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn invalidate_trampoline<P>(this: *mut ffi::GdkMonitor, f: glib_ffi::gpointer)
where P: IsA<Monitor> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Monitor::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_display_trampoline<P>(this: *mut ffi::GdkMonitor, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Monitor> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Monitor::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_geometry_trampoline<P>(this: *mut ffi::GdkMonitor, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Monitor> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Monitor::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_height_mm_trampoline<P>(this: *mut ffi::GdkMonitor, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Monitor> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Monitor::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_22", feature = "dox"))]
unsafe extern "C" fn notify_manufacturer_trampoline<P>(this: *mut ffi::GdkMonitor, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Monitor> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Monitor::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_22", feature = "dox"))]
unsafe extern "C" fn notify_model_trampoline<P>(this: *mut ffi::GdkMonitor, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Monitor> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Monitor::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_refresh_rate_trampoline<P>(this: *mut ffi::GdkMonitor, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Monitor> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Monitor::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_scale_factor_trampoline<P>(this: *mut ffi::GdkMonitor, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Monitor> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Monitor::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_22", feature = "dox"))]
unsafe extern "C" fn notify_subpixel_layout_trampoline<P>(this: *mut ffi::GdkMonitor, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Monitor> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Monitor::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_width_mm_trampoline<P>(this: *mut ffi::GdkMonitor, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Monitor> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Monitor::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_workarea_trampoline<P>(this: *mut ffi::GdkMonitor, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Monitor> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Monitor::from_glib_borrow(this).downcast_unchecked())
}
