// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from yolo_msgs:srv/StoreLandmark.idl
// generated code does not contain a copyright notice

#ifndef YOLO_MSGS__SRV__DETAIL__STORE_LANDMARK__STRUCT_HPP_
#define YOLO_MSGS__SRV__DETAIL__STORE_LANDMARK__STRUCT_HPP_

#include <algorithm>
#include <array>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "rosidl_runtime_cpp/bounded_vector.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


#ifndef _WIN32
# define DEPRECATED__yolo_msgs__srv__StoreLandmark_Request __attribute__((deprecated))
#else
# define DEPRECATED__yolo_msgs__srv__StoreLandmark_Request __declspec(deprecated)
#endif

namespace yolo_msgs
{

namespace srv
{

// message struct
template<class ContainerAllocator>
struct StoreLandmark_Request_
{
  using Type = StoreLandmark_Request_<ContainerAllocator>;

  explicit StoreLandmark_Request_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->label = "";
      this->x = 0.0f;
      this->y = 0.0f;
      this->z = 0.0f;
      this->confidence = 0.0f;
    }
  }

  explicit StoreLandmark_Request_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : label(_alloc)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->label = "";
      this->x = 0.0f;
      this->y = 0.0f;
      this->z = 0.0f;
      this->confidence = 0.0f;
    }
  }

  // field types and members
  using _label_type =
    std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>;
  _label_type label;
  using _x_type =
    float;
  _x_type x;
  using _y_type =
    float;
  _y_type y;
  using _z_type =
    float;
  _z_type z;
  using _confidence_type =
    float;
  _confidence_type confidence;

  // setters for named parameter idiom
  Type & set__label(
    const std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>> & _arg)
  {
    this->label = _arg;
    return *this;
  }
  Type & set__x(
    const float & _arg)
  {
    this->x = _arg;
    return *this;
  }
  Type & set__y(
    const float & _arg)
  {
    this->y = _arg;
    return *this;
  }
  Type & set__z(
    const float & _arg)
  {
    this->z = _arg;
    return *this;
  }
  Type & set__confidence(
    const float & _arg)
  {
    this->confidence = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    yolo_msgs::srv::StoreLandmark_Request_<ContainerAllocator> *;
  using ConstRawPtr =
    const yolo_msgs::srv::StoreLandmark_Request_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<yolo_msgs::srv::StoreLandmark_Request_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<yolo_msgs::srv::StoreLandmark_Request_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      yolo_msgs::srv::StoreLandmark_Request_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<yolo_msgs::srv::StoreLandmark_Request_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      yolo_msgs::srv::StoreLandmark_Request_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<yolo_msgs::srv::StoreLandmark_Request_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<yolo_msgs::srv::StoreLandmark_Request_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<yolo_msgs::srv::StoreLandmark_Request_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__yolo_msgs__srv__StoreLandmark_Request
    std::shared_ptr<yolo_msgs::srv::StoreLandmark_Request_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__yolo_msgs__srv__StoreLandmark_Request
    std::shared_ptr<yolo_msgs::srv::StoreLandmark_Request_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const StoreLandmark_Request_ & other) const
  {
    if (this->label != other.label) {
      return false;
    }
    if (this->x != other.x) {
      return false;
    }
    if (this->y != other.y) {
      return false;
    }
    if (this->z != other.z) {
      return false;
    }
    if (this->confidence != other.confidence) {
      return false;
    }
    return true;
  }
  bool operator!=(const StoreLandmark_Request_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct StoreLandmark_Request_

// alias to use template instance with default allocator
using StoreLandmark_Request =
  yolo_msgs::srv::StoreLandmark_Request_<std::allocator<void>>;

// constant definitions

}  // namespace srv

}  // namespace yolo_msgs


#ifndef _WIN32
# define DEPRECATED__yolo_msgs__srv__StoreLandmark_Response __attribute__((deprecated))
#else
# define DEPRECATED__yolo_msgs__srv__StoreLandmark_Response __declspec(deprecated)
#endif

namespace yolo_msgs
{

namespace srv
{

// message struct
template<class ContainerAllocator>
struct StoreLandmark_Response_
{
  using Type = StoreLandmark_Response_<ContainerAllocator>;

  explicit StoreLandmark_Response_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->success = false;
      this->message = "";
    }
  }

  explicit StoreLandmark_Response_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : message(_alloc)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->success = false;
      this->message = "";
    }
  }

  // field types and members
  using _success_type =
    bool;
  _success_type success;
  using _message_type =
    std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>;
  _message_type message;

  // setters for named parameter idiom
  Type & set__success(
    const bool & _arg)
  {
    this->success = _arg;
    return *this;
  }
  Type & set__message(
    const std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>> & _arg)
  {
    this->message = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    yolo_msgs::srv::StoreLandmark_Response_<ContainerAllocator> *;
  using ConstRawPtr =
    const yolo_msgs::srv::StoreLandmark_Response_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<yolo_msgs::srv::StoreLandmark_Response_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<yolo_msgs::srv::StoreLandmark_Response_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      yolo_msgs::srv::StoreLandmark_Response_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<yolo_msgs::srv::StoreLandmark_Response_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      yolo_msgs::srv::StoreLandmark_Response_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<yolo_msgs::srv::StoreLandmark_Response_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<yolo_msgs::srv::StoreLandmark_Response_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<yolo_msgs::srv::StoreLandmark_Response_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__yolo_msgs__srv__StoreLandmark_Response
    std::shared_ptr<yolo_msgs::srv::StoreLandmark_Response_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__yolo_msgs__srv__StoreLandmark_Response
    std::shared_ptr<yolo_msgs::srv::StoreLandmark_Response_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const StoreLandmark_Response_ & other) const
  {
    if (this->success != other.success) {
      return false;
    }
    if (this->message != other.message) {
      return false;
    }
    return true;
  }
  bool operator!=(const StoreLandmark_Response_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct StoreLandmark_Response_

// alias to use template instance with default allocator
using StoreLandmark_Response =
  yolo_msgs::srv::StoreLandmark_Response_<std::allocator<void>>;

// constant definitions

}  // namespace srv

}  // namespace yolo_msgs

namespace yolo_msgs
{

namespace srv
{

struct StoreLandmark
{
  using Request = yolo_msgs::srv::StoreLandmark_Request;
  using Response = yolo_msgs::srv::StoreLandmark_Response;
};

}  // namespace srv

}  // namespace yolo_msgs

#endif  // YOLO_MSGS__SRV__DETAIL__STORE_LANDMARK__STRUCT_HPP_
