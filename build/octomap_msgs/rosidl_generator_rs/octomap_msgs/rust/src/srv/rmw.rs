#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



#[link(name = "octomap_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__octomap_msgs__srv__BoundingBoxQuery_Request() -> *const std::ffi::c_void;
}

#[link(name = "octomap_msgs__rosidl_generator_c")]
extern "C" {
    fn octomap_msgs__srv__BoundingBoxQuery_Request__init(msg: *mut BoundingBoxQuery_Request) -> bool;
    fn octomap_msgs__srv__BoundingBoxQuery_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BoundingBoxQuery_Request>, size: usize) -> bool;
    fn octomap_msgs__srv__BoundingBoxQuery_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BoundingBoxQuery_Request>);
    fn octomap_msgs__srv__BoundingBoxQuery_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BoundingBoxQuery_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<BoundingBoxQuery_Request>) -> bool;
}

// Corresponds to octomap_msgs__srv__BoundingBoxQuery_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BoundingBoxQuery_Request {
    /// minimum corner point of axis-aligned bounding box in global frame
    pub min: geometry_msgs::msg::rmw::Point,

    /// maximum corner point of axis-aligned bounding box in global frame
    pub max: geometry_msgs::msg::rmw::Point,

}



impl Default for BoundingBoxQuery_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !octomap_msgs__srv__BoundingBoxQuery_Request__init(&mut msg as *mut _) {
        panic!("Call to octomap_msgs__srv__BoundingBoxQuery_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BoundingBoxQuery_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { octomap_msgs__srv__BoundingBoxQuery_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { octomap_msgs__srv__BoundingBoxQuery_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { octomap_msgs__srv__BoundingBoxQuery_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BoundingBoxQuery_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BoundingBoxQuery_Request where Self: Sized {
  const TYPE_NAME: &'static str = "octomap_msgs/srv/BoundingBoxQuery_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__octomap_msgs__srv__BoundingBoxQuery_Request() }
  }
}


#[link(name = "octomap_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__octomap_msgs__srv__BoundingBoxQuery_Response() -> *const std::ffi::c_void;
}

#[link(name = "octomap_msgs__rosidl_generator_c")]
extern "C" {
    fn octomap_msgs__srv__BoundingBoxQuery_Response__init(msg: *mut BoundingBoxQuery_Response) -> bool;
    fn octomap_msgs__srv__BoundingBoxQuery_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BoundingBoxQuery_Response>, size: usize) -> bool;
    fn octomap_msgs__srv__BoundingBoxQuery_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BoundingBoxQuery_Response>);
    fn octomap_msgs__srv__BoundingBoxQuery_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BoundingBoxQuery_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<BoundingBoxQuery_Response>) -> bool;
}

// Corresponds to octomap_msgs__srv__BoundingBoxQuery_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BoundingBoxQuery_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for BoundingBoxQuery_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !octomap_msgs__srv__BoundingBoxQuery_Response__init(&mut msg as *mut _) {
        panic!("Call to octomap_msgs__srv__BoundingBoxQuery_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BoundingBoxQuery_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { octomap_msgs__srv__BoundingBoxQuery_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { octomap_msgs__srv__BoundingBoxQuery_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { octomap_msgs__srv__BoundingBoxQuery_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BoundingBoxQuery_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BoundingBoxQuery_Response where Self: Sized {
  const TYPE_NAME: &'static str = "octomap_msgs/srv/BoundingBoxQuery_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__octomap_msgs__srv__BoundingBoxQuery_Response() }
  }
}


#[link(name = "octomap_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__octomap_msgs__srv__GetOctomap_Request() -> *const std::ffi::c_void;
}

#[link(name = "octomap_msgs__rosidl_generator_c")]
extern "C" {
    fn octomap_msgs__srv__GetOctomap_Request__init(msg: *mut GetOctomap_Request) -> bool;
    fn octomap_msgs__srv__GetOctomap_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetOctomap_Request>, size: usize) -> bool;
    fn octomap_msgs__srv__GetOctomap_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetOctomap_Request>);
    fn octomap_msgs__srv__GetOctomap_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetOctomap_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetOctomap_Request>) -> bool;
}

// Corresponds to octomap_msgs__srv__GetOctomap_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetOctomap_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for GetOctomap_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !octomap_msgs__srv__GetOctomap_Request__init(&mut msg as *mut _) {
        panic!("Call to octomap_msgs__srv__GetOctomap_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetOctomap_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { octomap_msgs__srv__GetOctomap_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { octomap_msgs__srv__GetOctomap_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { octomap_msgs__srv__GetOctomap_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetOctomap_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetOctomap_Request where Self: Sized {
  const TYPE_NAME: &'static str = "octomap_msgs/srv/GetOctomap_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__octomap_msgs__srv__GetOctomap_Request() }
  }
}


#[link(name = "octomap_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__octomap_msgs__srv__GetOctomap_Response() -> *const std::ffi::c_void;
}

#[link(name = "octomap_msgs__rosidl_generator_c")]
extern "C" {
    fn octomap_msgs__srv__GetOctomap_Response__init(msg: *mut GetOctomap_Response) -> bool;
    fn octomap_msgs__srv__GetOctomap_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetOctomap_Response>, size: usize) -> bool;
    fn octomap_msgs__srv__GetOctomap_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetOctomap_Response>);
    fn octomap_msgs__srv__GetOctomap_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetOctomap_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetOctomap_Response>) -> bool;
}

// Corresponds to octomap_msgs__srv__GetOctomap_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetOctomap_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub map: super::super::msg::rmw::Octomap,

}



impl Default for GetOctomap_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !octomap_msgs__srv__GetOctomap_Response__init(&mut msg as *mut _) {
        panic!("Call to octomap_msgs__srv__GetOctomap_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetOctomap_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { octomap_msgs__srv__GetOctomap_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { octomap_msgs__srv__GetOctomap_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { octomap_msgs__srv__GetOctomap_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetOctomap_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetOctomap_Response where Self: Sized {
  const TYPE_NAME: &'static str = "octomap_msgs/srv/GetOctomap_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__octomap_msgs__srv__GetOctomap_Response() }
  }
}






#[link(name = "octomap_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__octomap_msgs__srv__BoundingBoxQuery() -> *const std::ffi::c_void;
}

// Corresponds to octomap_msgs__srv__BoundingBoxQuery
#[allow(missing_docs, non_camel_case_types)]
pub struct BoundingBoxQuery;

impl rosidl_runtime_rs::Service for BoundingBoxQuery {
    type Request = BoundingBoxQuery_Request;
    type Response = BoundingBoxQuery_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__octomap_msgs__srv__BoundingBoxQuery() }
    }
}




#[link(name = "octomap_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__octomap_msgs__srv__GetOctomap() -> *const std::ffi::c_void;
}

// Corresponds to octomap_msgs__srv__GetOctomap
#[allow(missing_docs, non_camel_case_types)]
pub struct GetOctomap;

impl rosidl_runtime_rs::Service for GetOctomap {
    type Request = GetOctomap_Request;
    type Response = GetOctomap_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__octomap_msgs__srv__GetOctomap() }
    }
}


