#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to octomap_msgs__msg__Octomap
/// A 3D map in binary format, as Octree

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Octomap {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,

    /// Flag to denote a binary (only free/occupied) or full occupancy octree (.bt/.ot file)
    pub binary: bool,

    /// Class id of the contained octree
    pub id: std::string::String,

    /// Resolution (in m) of the smallest octree nodes
    pub resolution: f64,

    /// binary serialization of octree, use conversions.h to read and write octrees
    pub data: Vec<i8>,

}



impl Default for Octomap {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::Octomap::default())
  }
}

impl rosidl_runtime_rs::Message for Octomap {
  type RmwMsg = super::msg::rmw::Octomap;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        binary: msg.binary,
        id: msg.id.as_str().into(),
        resolution: msg.resolution,
        data: msg.data.into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
      binary: msg.binary,
        id: msg.id.as_str().into(),
      resolution: msg.resolution,
        data: msg.data.as_slice().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      binary: msg.binary,
      id: msg.id.to_string(),
      resolution: msg.resolution,
      data: msg.data
          .into_iter()
          .collect(),
    }
  }
}


// Corresponds to octomap_msgs__msg__OctomapWithPose
/// A 3D map in binary format, as Octree

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct OctomapWithPose {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,

    /// The pose of the octree with respect to the header frame
    pub origin: geometry_msgs::msg::Pose,

    /// The actual octree msg
    pub octomap: super::msg::Octomap,

}



impl Default for OctomapWithPose {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::OctomapWithPose::default())
  }
}

impl rosidl_runtime_rs::Message for OctomapWithPose {
  type RmwMsg = super::msg::rmw::OctomapWithPose;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        origin: geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Owned(msg.origin)).into_owned(),
        octomap: super::msg::Octomap::into_rmw_message(std::borrow::Cow::Owned(msg.octomap)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        origin: geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Borrowed(&msg.origin)).into_owned(),
        octomap: super::msg::Octomap::into_rmw_message(std::borrow::Cow::Borrowed(&msg.octomap)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      origin: geometry_msgs::msg::Pose::from_rmw_message(msg.origin),
      octomap: super::msg::Octomap::from_rmw_message(msg.octomap),
    }
  }
}


