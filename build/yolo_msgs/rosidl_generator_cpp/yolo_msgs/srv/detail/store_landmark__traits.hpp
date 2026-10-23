// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from yolo_msgs:srv/StoreLandmark.idl
// generated code does not contain a copyright notice

#ifndef YOLO_MSGS__SRV__DETAIL__STORE_LANDMARK__TRAITS_HPP_
#define YOLO_MSGS__SRV__DETAIL__STORE_LANDMARK__TRAITS_HPP_

#include <stdint.h>

#include <sstream>
#include <string>
#include <type_traits>

#include "yolo_msgs/srv/detail/store_landmark__struct.hpp"
#include "rosidl_runtime_cpp/traits.hpp"

namespace yolo_msgs
{

namespace srv
{

inline void to_flow_style_yaml(
  const StoreLandmark_Request & msg,
  std::ostream & out)
{
  out << "{";
  // member: label
  {
    out << "label: ";
    rosidl_generator_traits::value_to_yaml(msg.label, out);
    out << ", ";
  }

  // member: x
  {
    out << "x: ";
    rosidl_generator_traits::value_to_yaml(msg.x, out);
    out << ", ";
  }

  // member: y
  {
    out << "y: ";
    rosidl_generator_traits::value_to_yaml(msg.y, out);
    out << ", ";
  }

  // member: z
  {
    out << "z: ";
    rosidl_generator_traits::value_to_yaml(msg.z, out);
    out << ", ";
  }

  // member: confidence
  {
    out << "confidence: ";
    rosidl_generator_traits::value_to_yaml(msg.confidence, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const StoreLandmark_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: label
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "label: ";
    rosidl_generator_traits::value_to_yaml(msg.label, out);
    out << "\n";
  }

  // member: x
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "x: ";
    rosidl_generator_traits::value_to_yaml(msg.x, out);
    out << "\n";
  }

  // member: y
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "y: ";
    rosidl_generator_traits::value_to_yaml(msg.y, out);
    out << "\n";
  }

  // member: z
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "z: ";
    rosidl_generator_traits::value_to_yaml(msg.z, out);
    out << "\n";
  }

  // member: confidence
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "confidence: ";
    rosidl_generator_traits::value_to_yaml(msg.confidence, out);
    out << "\n";
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const StoreLandmark_Request & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace srv

}  // namespace yolo_msgs

namespace rosidl_generator_traits
{

[[deprecated("use yolo_msgs::srv::to_block_style_yaml() instead")]]
inline void to_yaml(
  const yolo_msgs::srv::StoreLandmark_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  yolo_msgs::srv::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use yolo_msgs::srv::to_yaml() instead")]]
inline std::string to_yaml(const yolo_msgs::srv::StoreLandmark_Request & msg)
{
  return yolo_msgs::srv::to_yaml(msg);
}

template<>
inline const char * data_type<yolo_msgs::srv::StoreLandmark_Request>()
{
  return "yolo_msgs::srv::StoreLandmark_Request";
}

template<>
inline const char * name<yolo_msgs::srv::StoreLandmark_Request>()
{
  return "yolo_msgs/srv/StoreLandmark_Request";
}

template<>
struct has_fixed_size<yolo_msgs::srv::StoreLandmark_Request>
  : std::integral_constant<bool, false> {};

template<>
struct has_bounded_size<yolo_msgs::srv::StoreLandmark_Request>
  : std::integral_constant<bool, false> {};

template<>
struct is_message<yolo_msgs::srv::StoreLandmark_Request>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace yolo_msgs
{

namespace srv
{

inline void to_flow_style_yaml(
  const StoreLandmark_Response & msg,
  std::ostream & out)
{
  out << "{";
  // member: success
  {
    out << "success: ";
    rosidl_generator_traits::value_to_yaml(msg.success, out);
    out << ", ";
  }

  // member: message
  {
    out << "message: ";
    rosidl_generator_traits::value_to_yaml(msg.message, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const StoreLandmark_Response & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: success
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "success: ";
    rosidl_generator_traits::value_to_yaml(msg.success, out);
    out << "\n";
  }

  // member: message
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "message: ";
    rosidl_generator_traits::value_to_yaml(msg.message, out);
    out << "\n";
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const StoreLandmark_Response & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace srv

}  // namespace yolo_msgs

namespace rosidl_generator_traits
{

[[deprecated("use yolo_msgs::srv::to_block_style_yaml() instead")]]
inline void to_yaml(
  const yolo_msgs::srv::StoreLandmark_Response & msg,
  std::ostream & out, size_t indentation = 0)
{
  yolo_msgs::srv::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use yolo_msgs::srv::to_yaml() instead")]]
inline std::string to_yaml(const yolo_msgs::srv::StoreLandmark_Response & msg)
{
  return yolo_msgs::srv::to_yaml(msg);
}

template<>
inline const char * data_type<yolo_msgs::srv::StoreLandmark_Response>()
{
  return "yolo_msgs::srv::StoreLandmark_Response";
}

template<>
inline const char * name<yolo_msgs::srv::StoreLandmark_Response>()
{
  return "yolo_msgs/srv/StoreLandmark_Response";
}

template<>
struct has_fixed_size<yolo_msgs::srv::StoreLandmark_Response>
  : std::integral_constant<bool, false> {};

template<>
struct has_bounded_size<yolo_msgs::srv::StoreLandmark_Response>
  : std::integral_constant<bool, false> {};

template<>
struct is_message<yolo_msgs::srv::StoreLandmark_Response>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace rosidl_generator_traits
{

template<>
inline const char * data_type<yolo_msgs::srv::StoreLandmark>()
{
  return "yolo_msgs::srv::StoreLandmark";
}

template<>
inline const char * name<yolo_msgs::srv::StoreLandmark>()
{
  return "yolo_msgs/srv/StoreLandmark";
}

template<>
struct has_fixed_size<yolo_msgs::srv::StoreLandmark>
  : std::integral_constant<
    bool,
    has_fixed_size<yolo_msgs::srv::StoreLandmark_Request>::value &&
    has_fixed_size<yolo_msgs::srv::StoreLandmark_Response>::value
  >
{
};

template<>
struct has_bounded_size<yolo_msgs::srv::StoreLandmark>
  : std::integral_constant<
    bool,
    has_bounded_size<yolo_msgs::srv::StoreLandmark_Request>::value &&
    has_bounded_size<yolo_msgs::srv::StoreLandmark_Response>::value
  >
{
};

template<>
struct is_service<yolo_msgs::srv::StoreLandmark>
  : std::true_type
{
};

template<>
struct is_service_request<yolo_msgs::srv::StoreLandmark_Request>
  : std::true_type
{
};

template<>
struct is_service_response<yolo_msgs::srv::StoreLandmark_Response>
  : std::true_type
{
};

}  // namespace rosidl_generator_traits

#endif  // YOLO_MSGS__SRV__DETAIL__STORE_LANDMARK__TRAITS_HPP_
