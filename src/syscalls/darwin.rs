// syscalls related to mac
use std::ffi::{ CString, CStr };
use std::os::raw::{ c_char, c_void };

type VoidPtr = *mut c_void;
type VoidFunction = extern "C" fn(VoidPtr, VoidPtr) -> VoidPtr;
type NsrectFunction = extern "C" fn(VoidPtr, VoidPtr) -> NSRect;
type StringFunction = extern "C" fn(VoidPtr, VoidPtr) -> *const c_char;

#[link(name = "objc")]
#[link(name = "AppKit", kind = "framework")]
#[link(name = "Foundation", kind = "framework")]
unsafe extern "C" {
    fn objc_getClass(name: *const c_char) -> VoidPtr;
    fn sel_registerName(name: *const c_char) -> VoidPtr;
    fn objc_msgSend() -> VoidPtr;
}

#[repr(C)]
#[derive(Clone, Copy, Default, Debug)]
pub struct NSPoint {
    x: f64,
    y: f64
}

#[repr(C)]
#[derive(Clone, Copy, Default, Debug)]
pub struct NSSize {
    width: f64,
    height: f64
}

#[repr(C)]
#[derive(Clone, Copy, Default, Debug)]
pub struct NSRect {
    origin: NSPoint,
    size: NSSize
}

pub fn get_screen_size() -> NSRect {
    unsafe {
        // name of the objc members/unit which are present in AppKit framework
        let nsscreen_class = CString::new("NSScreen").unwrap();
        let screen_property = CString::new("mainScreen").unwrap();
        let frame_property = CString::new("frame").unwrap(); 

        // a function pointer is created, it points to the objc_msgSend runtime function.
        // we're interpreting it with transmute, so that rust knows its a callable function.
        // objc_msgSend is a dispatcher, it dispatches our call to the receiver and
        // returns the result.
        let dispatcher: VoidFunction = std::mem::transmute(objc_msgSend as *const ());
        
        // creating a objc signature so that rust know it can point
        // and get the object pointer by matching the signature.
        // we need NSScreen class object to use as receiver so that we can get it screen size
        let receiver: VoidPtr = objc_getClass(nsscreen_class.as_ptr());

        // mainScreen method lookup in NSScreen.
        // it looksup the unit with same signature and returns the object pointer by executing it.
        let main_screen_property_pointer: VoidPtr = dispatcher(receiver, sel_registerName(screen_property.as_ptr()));

        // since, the return type of frame is NSRect,
        // we cannot use the same dispatcher which signaure is (void, void) -> void.
        // we must use and match the actual ABI.
        // so, creating another dispatcher with (void, void) -> NSRect signature.
        let frame_dispatcher: NsrectFunction = std::mem::transmute(objc_msgSend as *const ());

        // we already have a object pointer to NSScreen.mainScreen,
        // looking the signature and returns the result by executing NSScreen.mainScreen.frame
        frame_dispatcher(main_screen_property_pointer, sel_registerName(frame_property.as_ptr()))
    }
}

pub fn get_process_info() -> (String, String, String) {

    unsafe {
        let nsprocess_info_class = CString::new("NSProcessInfo").unwrap();
        let process_info_property = CString::new("processInfo").unwrap();
        let username_property = CString::new("userName").unwrap();
        let full_username_property = CString::new("fullUserName").unwrap();
        let hostname = CString::new("hostName").unwrap();
        let utf8_property = CString::new("UTF8String").unwrap();

        let dispatcher: VoidFunction = std::mem::transmute(objc_msgSend as *const());
        let receiver: VoidPtr = objc_getClass(nsprocess_info_class.as_ptr());
        let process_info_receiver = dispatcher(receiver, sel_registerName(process_info_property.as_ptr()));

        let username_dispatcher: VoidFunction = std::mem::transmute(objc_msgSend as *const());

        let username_receiver = username_dispatcher(process_info_receiver, sel_registerName(username_property.as_ptr()));

        let full_username_receiver = username_dispatcher(process_info_receiver, sel_registerName(full_username_property.as_ptr()));

        let host_name_receiver = username_dispatcher(process_info_receiver, sel_registerName(hostname.as_ptr()));

        let utf8_dispatcher: StringFunction = std::mem::transmute(objc_msgSend as *const());
        let username_raw_data = utf8_dispatcher(username_receiver, sel_registerName(utf8_property.as_ptr()));
        let full_username_raw_data = utf8_dispatcher(full_username_receiver, sel_registerName(utf8_property.as_ptr()));
        let host_name_raw_data = utf8_dispatcher(host_name_receiver, sel_registerName(utf8_property.as_ptr()));

        (
            String::from(CStr::from_ptr(username_raw_data).to_str().unwrap()),
            String::from(CStr::from_ptr(full_username_raw_data).to_str().unwrap()),
            String::from(CStr::from_ptr(host_name_raw_data).to_str().unwrap())
        )
    }
}
