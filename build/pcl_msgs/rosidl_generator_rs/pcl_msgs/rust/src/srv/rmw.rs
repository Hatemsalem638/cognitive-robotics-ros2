#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



#[link(name = "pcl_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__pcl_msgs__srv__UpdateFilename_Request() -> *const std::ffi::c_void;
}

#[link(name = "pcl_msgs__rosidl_generator_c")]
extern "C" {
    fn pcl_msgs__srv__UpdateFilename_Request__init(msg: *mut UpdateFilename_Request) -> bool;
    fn pcl_msgs__srv__UpdateFilename_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<UpdateFilename_Request>, size: usize) -> bool;
    fn pcl_msgs__srv__UpdateFilename_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<UpdateFilename_Request>);
    fn pcl_msgs__srv__UpdateFilename_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<UpdateFilename_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<UpdateFilename_Request>) -> bool;
}

// Corresponds to pcl_msgs__srv__UpdateFilename_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct UpdateFilename_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub filename: rosidl_runtime_rs::String,

}



impl Default for UpdateFilename_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !pcl_msgs__srv__UpdateFilename_Request__init(&mut msg as *mut _) {
        panic!("Call to pcl_msgs__srv__UpdateFilename_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for UpdateFilename_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { pcl_msgs__srv__UpdateFilename_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { pcl_msgs__srv__UpdateFilename_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { pcl_msgs__srv__UpdateFilename_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for UpdateFilename_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for UpdateFilename_Request where Self: Sized {
  const TYPE_NAME: &'static str = "pcl_msgs/srv/UpdateFilename_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__pcl_msgs__srv__UpdateFilename_Request() }
  }
}


#[link(name = "pcl_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__pcl_msgs__srv__UpdateFilename_Response() -> *const std::ffi::c_void;
}

#[link(name = "pcl_msgs__rosidl_generator_c")]
extern "C" {
    fn pcl_msgs__srv__UpdateFilename_Response__init(msg: *mut UpdateFilename_Response) -> bool;
    fn pcl_msgs__srv__UpdateFilename_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<UpdateFilename_Response>, size: usize) -> bool;
    fn pcl_msgs__srv__UpdateFilename_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<UpdateFilename_Response>);
    fn pcl_msgs__srv__UpdateFilename_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<UpdateFilename_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<UpdateFilename_Response>) -> bool;
}

// Corresponds to pcl_msgs__srv__UpdateFilename_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct UpdateFilename_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

}



impl Default for UpdateFilename_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !pcl_msgs__srv__UpdateFilename_Response__init(&mut msg as *mut _) {
        panic!("Call to pcl_msgs__srv__UpdateFilename_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for UpdateFilename_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { pcl_msgs__srv__UpdateFilename_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { pcl_msgs__srv__UpdateFilename_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { pcl_msgs__srv__UpdateFilename_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for UpdateFilename_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for UpdateFilename_Response where Self: Sized {
  const TYPE_NAME: &'static str = "pcl_msgs/srv/UpdateFilename_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__pcl_msgs__srv__UpdateFilename_Response() }
  }
}






#[link(name = "pcl_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__pcl_msgs__srv__UpdateFilename() -> *const std::ffi::c_void;
}

// Corresponds to pcl_msgs__srv__UpdateFilename
#[allow(missing_docs, non_camel_case_types)]
pub struct UpdateFilename;

impl rosidl_runtime_rs::Service for UpdateFilename {
    type Request = UpdateFilename_Request;
    type Response = UpdateFilename_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__pcl_msgs__srv__UpdateFilename() }
    }
}


