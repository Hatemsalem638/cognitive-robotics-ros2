// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from yolo_msgs:msg/Detection.idl
// generated code does not contain a copyright notice

#ifndef YOLO_MSGS__MSG__DETAIL__DETECTION__STRUCT_H_
#define YOLO_MSGS__MSG__DETAIL__DETECTION__STRUCT_H_

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

/// Struct defined in msg/Detection in the package yolo_msgs.
typedef struct yolo_msgs__msg__Detection
{
  rosidl_runtime_c__String label;
  float confidence;
  float x1;
  float y1;
  float x2;
  float y2;
} yolo_msgs__msg__Detection;

// Struct for a sequence of yolo_msgs__msg__Detection.
typedef struct yolo_msgs__msg__Detection__Sequence
{
  yolo_msgs__msg__Detection * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} yolo_msgs__msg__Detection__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // YOLO_MSGS__MSG__DETAIL__DETECTION__STRUCT_H_
