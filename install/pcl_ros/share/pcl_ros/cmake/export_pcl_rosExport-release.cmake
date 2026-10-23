#----------------------------------------------------------------
# Generated CMake target import file for configuration "Release".
#----------------------------------------------------------------

# Commands may need to know the format version.
set(CMAKE_IMPORT_FILE_VERSION 1)

# Import target "pcl_ros::pcl_ros_tf" for configuration "Release"
set_property(TARGET pcl_ros::pcl_ros_tf APPEND PROPERTY IMPORTED_CONFIGURATIONS RELEASE)
set_target_properties(pcl_ros::pcl_ros_tf PROPERTIES
  IMPORTED_LINK_INTERFACE_LANGUAGES_RELEASE "CXX"
  IMPORTED_LOCATION_RELEASE "${_IMPORT_PREFIX}/lib/libpcl_ros_tf.a"
  )

list(APPEND _IMPORT_CHECK_TARGETS pcl_ros::pcl_ros_tf )
list(APPEND _IMPORT_CHECK_FILES_FOR_pcl_ros::pcl_ros_tf "${_IMPORT_PREFIX}/lib/libpcl_ros_tf.a" )

# Import target "pcl_ros::pcd_to_pointcloud_lib" for configuration "Release"
set_property(TARGET pcl_ros::pcd_to_pointcloud_lib APPEND PROPERTY IMPORTED_CONFIGURATIONS RELEASE)
set_target_properties(pcl_ros::pcd_to_pointcloud_lib PROPERTIES
  IMPORTED_LOCATION_RELEASE "${_IMPORT_PREFIX}/lib/libpcd_to_pointcloud_lib.so"
  IMPORTED_SONAME_RELEASE "libpcd_to_pointcloud_lib.so"
  )

list(APPEND _IMPORT_CHECK_TARGETS pcl_ros::pcd_to_pointcloud_lib )
list(APPEND _IMPORT_CHECK_FILES_FOR_pcl_ros::pcd_to_pointcloud_lib "${_IMPORT_PREFIX}/lib/libpcd_to_pointcloud_lib.so" )

# Import target "pcl_ros::pcl_ros_filters" for configuration "Release"
set_property(TARGET pcl_ros::pcl_ros_filters APPEND PROPERTY IMPORTED_CONFIGURATIONS RELEASE)
set_target_properties(pcl_ros::pcl_ros_filters PROPERTIES
  IMPORTED_LOCATION_RELEASE "${_IMPORT_PREFIX}/lib/libpcl_ros_filters.so"
  IMPORTED_SONAME_RELEASE "libpcl_ros_filters.so"
  )

list(APPEND _IMPORT_CHECK_TARGETS pcl_ros::pcl_ros_filters )
list(APPEND _IMPORT_CHECK_FILES_FOR_pcl_ros::pcl_ros_filters "${_IMPORT_PREFIX}/lib/libpcl_ros_filters.so" )

# Import target "pcl_ros::pointcloud_to_pcd_lib" for configuration "Release"
set_property(TARGET pcl_ros::pointcloud_to_pcd_lib APPEND PROPERTY IMPORTED_CONFIGURATIONS RELEASE)
set_target_properties(pcl_ros::pointcloud_to_pcd_lib PROPERTIES
  IMPORTED_LOCATION_RELEASE "${_IMPORT_PREFIX}/lib/libpointcloud_to_pcd_lib.so"
  IMPORTED_SONAME_RELEASE "libpointcloud_to_pcd_lib.so"
  )

list(APPEND _IMPORT_CHECK_TARGETS pcl_ros::pointcloud_to_pcd_lib )
list(APPEND _IMPORT_CHECK_FILES_FOR_pcl_ros::pointcloud_to_pcd_lib "${_IMPORT_PREFIX}/lib/libpointcloud_to_pcd_lib.so" )

# Commands beyond this point should not need to know the version.
set(CMAKE_IMPORT_FILE_VERSION)
