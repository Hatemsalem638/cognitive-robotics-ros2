// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from yolo_msgs:srv/StoreLandmark.idl
// generated code does not contain a copyright notice

#ifndef YOLO_MSGS__SRV__DETAIL__STORE_LANDMARK__BUILDER_HPP_
#define YOLO_MSGS__SRV__DETAIL__STORE_LANDMARK__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "yolo_msgs/srv/detail/store_landmark__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace yolo_msgs
{

namespace srv
{

namespace builder
{

class Init_StoreLandmark_Request_confidence
{
public:
  explicit Init_StoreLandmark_Request_confidence(::yolo_msgs::srv::StoreLandmark_Request & msg)
  : msg_(msg)
  {}
  ::yolo_msgs::srv::StoreLandmark_Request confidence(::yolo_msgs::srv::StoreLandmark_Request::_confidence_type arg)
  {
    msg_.confidence = std::move(arg);
    return std::move(msg_);
  }

private:
  ::yolo_msgs::srv::StoreLandmark_Request msg_;
};

class Init_StoreLandmark_Request_z
{
public:
  explicit Init_StoreLandmark_Request_z(::yolo_msgs::srv::StoreLandmark_Request & msg)
  : msg_(msg)
  {}
  Init_StoreLandmark_Request_confidence z(::yolo_msgs::srv::StoreLandmark_Request::_z_type arg)
  {
    msg_.z = std::move(arg);
    return Init_StoreLandmark_Request_confidence(msg_);
  }

private:
  ::yolo_msgs::srv::StoreLandmark_Request msg_;
};

class Init_StoreLandmark_Request_y
{
public:
  explicit Init_StoreLandmark_Request_y(::yolo_msgs::srv::StoreLandmark_Request & msg)
  : msg_(msg)
  {}
  Init_StoreLandmark_Request_z y(::yolo_msgs::srv::StoreLandmark_Request::_y_type arg)
  {
    msg_.y = std::move(arg);
    return Init_StoreLandmark_Request_z(msg_);
  }

private:
  ::yolo_msgs::srv::StoreLandmark_Request msg_;
};

class Init_StoreLandmark_Request_x
{
public:
  explicit Init_StoreLandmark_Request_x(::yolo_msgs::srv::StoreLandmark_Request & msg)
  : msg_(msg)
  {}
  Init_StoreLandmark_Request_y x(::yolo_msgs::srv::StoreLandmark_Request::_x_type arg)
  {
    msg_.x = std::move(arg);
    return Init_StoreLandmark_Request_y(msg_);
  }

private:
  ::yolo_msgs::srv::StoreLandmark_Request msg_;
};

class Init_StoreLandmark_Request_label
{
public:
  Init_StoreLandmark_Request_label()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_StoreLandmark_Request_x label(::yolo_msgs::srv::StoreLandmark_Request::_label_type arg)
  {
    msg_.label = std::move(arg);
    return Init_StoreLandmark_Request_x(msg_);
  }

private:
  ::yolo_msgs::srv::StoreLandmark_Request msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::yolo_msgs::srv::StoreLandmark_Request>()
{
  return yolo_msgs::srv::builder::Init_StoreLandmark_Request_label();
}

}  // namespace yolo_msgs


namespace yolo_msgs
{

namespace srv
{

namespace builder
{

class Init_StoreLandmark_Response_message
{
public:
  explicit Init_StoreLandmark_Response_message(::yolo_msgs::srv::StoreLandmark_Response & msg)
  : msg_(msg)
  {}
  ::yolo_msgs::srv::StoreLandmark_Response message(::yolo_msgs::srv::StoreLandmark_Response::_message_type arg)
  {
    msg_.message = std::move(arg);
    return std::move(msg_);
  }

private:
  ::yolo_msgs::srv::StoreLandmark_Response msg_;
};

class Init_StoreLandmark_Response_success
{
public:
  Init_StoreLandmark_Response_success()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_StoreLandmark_Response_message success(::yolo_msgs::srv::StoreLandmark_Response::_success_type arg)
  {
    msg_.success = std::move(arg);
    return Init_StoreLandmark_Response_message(msg_);
  }

private:
  ::yolo_msgs::srv::StoreLandmark_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::yolo_msgs::srv::StoreLandmark_Response>()
{
  return yolo_msgs::srv::builder::Init_StoreLandmark_Response_success();
}

}  // namespace yolo_msgs

#endif  // YOLO_MSGS__SRV__DETAIL__STORE_LANDMARK__BUILDER_HPP_
