// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from yolo_msgs:srv/StoreLandmark.idl
// generated code does not contain a copyright notice

#ifndef YOLO_MSGS__SRV__DETAIL__STORE_LANDMARK__STRUCT_H_
#define YOLO_MSGS__SRV__DETAIL__STORE_LANDMARK__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

// Include directives for member types
// Member 'label'
#include "rosidl_runtime_c/string.h"

/// Struct defined in srv/StoreLandmark in the package yolo_msgs.
typedef struct yolo_msgs__srv__StoreLandmark_Request
{
  rosidl_runtime_c__String label;
  float x;
  float y;
  float z;
  float confidence;
} yolo_msgs__srv__StoreLandmark_Request;

// Struct for a sequence of yolo_msgs__srv__StoreLandmark_Request.
typedef struct yolo_msgs__srv__StoreLandmark_Request__Sequence
{
  yolo_msgs__srv__StoreLandmark_Request * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} yolo_msgs__srv__StoreLandmark_Request__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'message'
// already included above
// #include "rosidl_runtime_c/string.h"

/// Struct defined in srv/StoreLandmark in the package yolo_msgs.
typedef struct yolo_msgs__srv__StoreLandmark_Response
{
  bool success;
  rosidl_runtime_c__String message;
} yolo_msgs__srv__StoreLandmark_Response;

// Struct for a sequence of yolo_msgs__srv__StoreLandmark_Response.
typedef struct yolo_msgs__srv__StoreLandmark_Response__Sequence
{
  yolo_msgs__srv__StoreLandmark_Response * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} yolo_msgs__srv__StoreLandmark_Response__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // YOLO_MSGS__SRV__DETAIL__STORE_LANDMARK__STRUCT_H_
