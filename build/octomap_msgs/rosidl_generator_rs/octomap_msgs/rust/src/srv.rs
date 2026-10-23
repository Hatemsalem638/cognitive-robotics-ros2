#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};




// Corresponds to octomap_msgs__srv__BoundingBoxQuery_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BoundingBoxQuery_Request {
    /// minimum corner point of axis-aligned bounding box in global frame
    pub min: geometry_msgs::msg::Point,

    /// maximum corner point of axis-aligned bounding box in global frame
    pub max: geometry_msgs::msg::Point,

}



impl Default for BoundingBoxQuery_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::BoundingBoxQuery_Request::default())
  }
}

impl rosidl_runtime_rs::Message for BoundingBoxQuery_Request {
  type RmwMsg = super::srv::rmw::BoundingBoxQuery_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        min: geometry_msgs::msg::Point::into_rmw_message(std::borrow::Cow::Owned(msg.min)).into_owned(),
        max: geometry_msgs::msg::Point::into_rmw_message(std::borrow::Cow::Owned(msg.max)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        min: geometry_msgs::msg::Point::into_rmw_message(std::borrow::Cow::Borrowed(&msg.min)).into_owned(),
        max: geometry_msgs::msg::Point::into_rmw_message(std::borrow::Cow::Borrowed(&msg.max)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      min: geometry_msgs::msg::Point::from_rmw_message(msg.min),
      max: geometry_msgs::msg::Point::from_rmw_message(msg.max),
    }
  }
}


// Corresponds to octomap_msgs__srv__BoundingBoxQuery_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BoundingBoxQuery_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for BoundingBoxQuery_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::BoundingBoxQuery_Response::default())
  }
}

impl rosidl_runtime_rs::Message for BoundingBoxQuery_Response {
  type RmwMsg = super::srv::rmw::BoundingBoxQuery_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to octomap_msgs__srv__GetOctomap_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetOctomap_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for GetOctomap_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetOctomap_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetOctomap_Request {
  type RmwMsg = super::srv::rmw::GetOctomap_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to octomap_msgs__srv__GetOctomap_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetOctomap_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub map: super::msg::Octomap,

}



impl Default for GetOctomap_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetOctomap_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetOctomap_Response {
  type RmwMsg = super::srv::rmw::GetOctomap_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        map: super::msg::Octomap::into_rmw_message(std::borrow::Cow::Owned(msg.map)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        map: super::msg::Octomap::into_rmw_message(std::borrow::Cow::Borrowed(&msg.map)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      map: super::msg::Octomap::from_rmw_message(msg.map),
    }
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


