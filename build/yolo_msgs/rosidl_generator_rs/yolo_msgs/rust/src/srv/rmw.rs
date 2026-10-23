#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



#[link(name = "yolo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__yolo_msgs__srv__StoreLandmark_Request() -> *const std::ffi::c_void;
}

#[link(name = "yolo_msgs__rosidl_generator_c")]
extern "C" {
    fn yolo_msgs__srv__StoreLandmark_Request__init(msg: *mut StoreLandmark_Request) -> bool;
    fn yolo_msgs__srv__StoreLandmark_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<StoreLandmark_Request>, size: usize) -> bool;
    fn yolo_msgs__srv__StoreLandmark_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<StoreLandmark_Request>);
    fn yolo_msgs__srv__StoreLandmark_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<StoreLandmark_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<StoreLandmark_Request>) -> bool;
}

// Corresponds to yolo_msgs__srv__StoreLandmark_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct StoreLandmark_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub label: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub x: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub y: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub z: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub confidence: f32,

}



impl Default for StoreLandmark_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !yolo_msgs__srv__StoreLandmark_Request__init(&mut msg as *mut _) {
        panic!("Call to yolo_msgs__srv__StoreLandmark_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for StoreLandmark_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { yolo_msgs__srv__StoreLandmark_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { yolo_msgs__srv__StoreLandmark_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { yolo_msgs__srv__StoreLandmark_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for StoreLandmark_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for StoreLandmark_Request where Self: Sized {
  const TYPE_NAME: &'static str = "yolo_msgs/srv/StoreLandmark_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__yolo_msgs__srv__StoreLandmark_Request() }
  }
}


#[link(name = "yolo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__yolo_msgs__srv__StoreLandmark_Response() -> *const std::ffi::c_void;
}

#[link(name = "yolo_msgs__rosidl_generator_c")]
extern "C" {
    fn yolo_msgs__srv__StoreLandmark_Response__init(msg: *mut StoreLandmark_Response) -> bool;
    fn yolo_msgs__srv__StoreLandmark_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<StoreLandmark_Response>, size: usize) -> bool;
    fn yolo_msgs__srv__StoreLandmark_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<StoreLandmark_Response>);
    fn yolo_msgs__srv__StoreLandmark_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<StoreLandmark_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<StoreLandmark_Response>) -> bool;
}

// Corresponds to yolo_msgs__srv__StoreLandmark_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct StoreLandmark_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,

}



impl Default for StoreLandmark_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !yolo_msgs__srv__StoreLandmark_Response__init(&mut msg as *mut _) {
        panic!("Call to yolo_msgs__srv__StoreLandmark_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for StoreLandmark_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { yolo_msgs__srv__StoreLandmark_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { yolo_msgs__srv__StoreLandmark_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { yolo_msgs__srv__StoreLandmark_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for StoreLandmark_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for StoreLandmark_Response where Self: Sized {
  const TYPE_NAME: &'static str = "yolo_msgs/srv/StoreLandmark_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__yolo_msgs__srv__StoreLandmark_Response() }
  }
}






#[link(name = "yolo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__yolo_msgs__srv__StoreLandmark() -> *const std::ffi::c_void;
}

// Corresponds to yolo_msgs__srv__StoreLandmark
#[allow(missing_docs, non_camel_case_types)]
pub struct StoreLandmark;

impl rosidl_runtime_rs::Service for StoreLandmark {
    type Request = StoreLandmark_Request;
    type Response = StoreLandmark_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__yolo_msgs__srv__StoreLandmark() }
    }
}


