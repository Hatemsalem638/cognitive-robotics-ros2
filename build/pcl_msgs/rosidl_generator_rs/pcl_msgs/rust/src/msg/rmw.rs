#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "pcl_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__pcl_msgs__msg__ModelCoefficients() -> *const std::ffi::c_void;
}

#[link(name = "pcl_msgs__rosidl_generator_c")]
extern "C" {
    fn pcl_msgs__msg__ModelCoefficients__init(msg: *mut ModelCoefficients) -> bool;
    fn pcl_msgs__msg__ModelCoefficients__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ModelCoefficients>, size: usize) -> bool;
    fn pcl_msgs__msg__ModelCoefficients__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ModelCoefficients>);
    fn pcl_msgs__msg__ModelCoefficients__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ModelCoefficients>, out_seq: *mut rosidl_runtime_rs::Sequence<ModelCoefficients>) -> bool;
}

// Corresponds to pcl_msgs__msg__ModelCoefficients
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ModelCoefficients {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub values: rosidl_runtime_rs::Sequence<f32>,

}



impl Default for ModelCoefficients {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !pcl_msgs__msg__ModelCoefficients__init(&mut msg as *mut _) {
        panic!("Call to pcl_msgs__msg__ModelCoefficients__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ModelCoefficients {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { pcl_msgs__msg__ModelCoefficients__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { pcl_msgs__msg__ModelCoefficients__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { pcl_msgs__msg__ModelCoefficients__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ModelCoefficients {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ModelCoefficients where Self: Sized {
  const TYPE_NAME: &'static str = "pcl_msgs/msg/ModelCoefficients";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__pcl_msgs__msg__ModelCoefficients() }
  }
}


#[link(name = "pcl_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__pcl_msgs__msg__PointIndices() -> *const std::ffi::c_void;
}

#[link(name = "pcl_msgs__rosidl_generator_c")]
extern "C" {
    fn pcl_msgs__msg__PointIndices__init(msg: *mut PointIndices) -> bool;
    fn pcl_msgs__msg__PointIndices__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PointIndices>, size: usize) -> bool;
    fn pcl_msgs__msg__PointIndices__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PointIndices>);
    fn pcl_msgs__msg__PointIndices__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PointIndices>, out_seq: *mut rosidl_runtime_rs::Sequence<PointIndices>) -> bool;
}

// Corresponds to pcl_msgs__msg__PointIndices
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PointIndices {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub indices: rosidl_runtime_rs::Sequence<i32>,

}



impl Default for PointIndices {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !pcl_msgs__msg__PointIndices__init(&mut msg as *mut _) {
        panic!("Call to pcl_msgs__msg__PointIndices__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PointIndices {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { pcl_msgs__msg__PointIndices__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { pcl_msgs__msg__PointIndices__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { pcl_msgs__msg__PointIndices__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PointIndices {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PointIndices where Self: Sized {
  const TYPE_NAME: &'static str = "pcl_msgs/msg/PointIndices";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__pcl_msgs__msg__PointIndices() }
  }
}


#[link(name = "pcl_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__pcl_msgs__msg__PolygonMesh() -> *const std::ffi::c_void;
}

#[link(name = "pcl_msgs__rosidl_generator_c")]
extern "C" {
    fn pcl_msgs__msg__PolygonMesh__init(msg: *mut PolygonMesh) -> bool;
    fn pcl_msgs__msg__PolygonMesh__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PolygonMesh>, size: usize) -> bool;
    fn pcl_msgs__msg__PolygonMesh__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PolygonMesh>);
    fn pcl_msgs__msg__PolygonMesh__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PolygonMesh>, out_seq: *mut rosidl_runtime_rs::Sequence<PolygonMesh>) -> bool;
}

// Corresponds to pcl_msgs__msg__PolygonMesh
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Separate header for the polygonal surface

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PolygonMesh {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,

    /// Vertices of the mesh as a point cloud
    pub cloud: sensor_msgs::msg::rmw::PointCloud2,

    /// List of polygons
    pub polygons: rosidl_runtime_rs::Sequence<super::super::msg::rmw::Vertices>,

}



impl Default for PolygonMesh {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !pcl_msgs__msg__PolygonMesh__init(&mut msg as *mut _) {
        panic!("Call to pcl_msgs__msg__PolygonMesh__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PolygonMesh {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { pcl_msgs__msg__PolygonMesh__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { pcl_msgs__msg__PolygonMesh__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { pcl_msgs__msg__PolygonMesh__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PolygonMesh {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PolygonMesh where Self: Sized {
  const TYPE_NAME: &'static str = "pcl_msgs/msg/PolygonMesh";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__pcl_msgs__msg__PolygonMesh() }
  }
}


#[link(name = "pcl_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__pcl_msgs__msg__Vertices() -> *const std::ffi::c_void;
}

#[link(name = "pcl_msgs__rosidl_generator_c")]
extern "C" {
    fn pcl_msgs__msg__Vertices__init(msg: *mut Vertices) -> bool;
    fn pcl_msgs__msg__Vertices__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Vertices>, size: usize) -> bool;
    fn pcl_msgs__msg__Vertices__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Vertices>);
    fn pcl_msgs__msg__Vertices__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Vertices>, out_seq: *mut rosidl_runtime_rs::Sequence<Vertices>) -> bool;
}

// Corresponds to pcl_msgs__msg__Vertices
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// List of point indices

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Vertices {

    // This member is not documented.
    #[allow(missing_docs)]
    pub vertices: rosidl_runtime_rs::Sequence<u32>,

}



impl Default for Vertices {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !pcl_msgs__msg__Vertices__init(&mut msg as *mut _) {
        panic!("Call to pcl_msgs__msg__Vertices__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Vertices {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { pcl_msgs__msg__Vertices__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { pcl_msgs__msg__Vertices__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { pcl_msgs__msg__Vertices__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Vertices {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Vertices where Self: Sized {
  const TYPE_NAME: &'static str = "pcl_msgs/msg/Vertices";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__pcl_msgs__msg__Vertices() }
  }
}


