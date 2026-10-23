#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "octomap_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__octomap_msgs__msg__Octomap() -> *const std::ffi::c_void;
}

#[link(name = "octomap_msgs__rosidl_generator_c")]
extern "C" {
    fn octomap_msgs__msg__Octomap__init(msg: *mut Octomap) -> bool;
    fn octomap_msgs__msg__Octomap__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Octomap>, size: usize) -> bool;
    fn octomap_msgs__msg__Octomap__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Octomap>);
    fn octomap_msgs__msg__Octomap__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Octomap>, out_seq: *mut rosidl_runtime_rs::Sequence<Octomap>) -> bool;
}

// Corresponds to octomap_msgs__msg__Octomap
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// A 3D map in binary format, as Octree

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Octomap {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,

    /// Flag to denote a binary (only free/occupied) or full occupancy octree (.bt/.ot file)
    pub binary: bool,

    /// Class id of the contained octree
    pub id: rosidl_runtime_rs::String,

    /// Resolution (in m) of the smallest octree nodes
    pub resolution: f64,

    /// binary serialization of octree, use conversions.h to read and write octrees
    pub data: rosidl_runtime_rs::Sequence<i8>,

}



impl Default for Octomap {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !octomap_msgs__msg__Octomap__init(&mut msg as *mut _) {
        panic!("Call to octomap_msgs__msg__Octomap__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Octomap {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { octomap_msgs__msg__Octomap__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { octomap_msgs__msg__Octomap__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { octomap_msgs__msg__Octomap__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Octomap {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Octomap where Self: Sized {
  const TYPE_NAME: &'static str = "octomap_msgs/msg/Octomap";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__octomap_msgs__msg__Octomap() }
  }
}


#[link(name = "octomap_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__octomap_msgs__msg__OctomapWithPose() -> *const std::ffi::c_void;
}

#[link(name = "octomap_msgs__rosidl_generator_c")]
extern "C" {
    fn octomap_msgs__msg__OctomapWithPose__init(msg: *mut OctomapWithPose) -> bool;
    fn octomap_msgs__msg__OctomapWithPose__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<OctomapWithPose>, size: usize) -> bool;
    fn octomap_msgs__msg__OctomapWithPose__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<OctomapWithPose>);
    fn octomap_msgs__msg__OctomapWithPose__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<OctomapWithPose>, out_seq: *mut rosidl_runtime_rs::Sequence<OctomapWithPose>) -> bool;
}

// Corresponds to octomap_msgs__msg__OctomapWithPose
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// A 3D map in binary format, as Octree

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct OctomapWithPose {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,

    /// The pose of the octree with respect to the header frame
    pub origin: geometry_msgs::msg::rmw::Pose,

    /// The actual octree msg
    pub octomap: super::super::msg::rmw::Octomap,

}



impl Default for OctomapWithPose {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !octomap_msgs__msg__OctomapWithPose__init(&mut msg as *mut _) {
        panic!("Call to octomap_msgs__msg__OctomapWithPose__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for OctomapWithPose {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { octomap_msgs__msg__OctomapWithPose__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { octomap_msgs__msg__OctomapWithPose__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { octomap_msgs__msg__OctomapWithPose__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for OctomapWithPose {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for OctomapWithPose where Self: Sized {
  const TYPE_NAME: &'static str = "octomap_msgs/msg/OctomapWithPose";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__octomap_msgs__msg__OctomapWithPose() }
  }
}


