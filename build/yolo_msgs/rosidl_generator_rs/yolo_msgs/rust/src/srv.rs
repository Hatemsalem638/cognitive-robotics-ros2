#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};




// Corresponds to yolo_msgs__srv__StoreLandmark_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct StoreLandmark_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub label: std::string::String,


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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::StoreLandmark_Request::default())
  }
}

impl rosidl_runtime_rs::Message for StoreLandmark_Request {
  type RmwMsg = super::srv::rmw::StoreLandmark_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        label: msg.label.as_str().into(),
        x: msg.x,
        y: msg.y,
        z: msg.z,
        confidence: msg.confidence,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        label: msg.label.as_str().into(),
      x: msg.x,
      y: msg.y,
      z: msg.z,
      confidence: msg.confidence,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      label: msg.label.to_string(),
      x: msg.x,
      y: msg.y,
      z: msg.z,
      confidence: msg.confidence,
    }
  }
}


// Corresponds to yolo_msgs__srv__StoreLandmark_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct StoreLandmark_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: std::string::String,

}



impl Default for StoreLandmark_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::StoreLandmark_Response::default())
  }
}

impl rosidl_runtime_rs::Message for StoreLandmark_Response {
  type RmwMsg = super::srv::rmw::StoreLandmark_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        message: msg.message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        message: msg.message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      message: msg.message.to_string(),
    }
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


