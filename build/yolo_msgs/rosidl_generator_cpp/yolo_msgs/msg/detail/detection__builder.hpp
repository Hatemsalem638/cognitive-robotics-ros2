// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from yolo_msgs:msg/Detection.idl
// generated code does not contain a copyright notice

#ifndef YOLO_MSGS__MSG__DETAIL__DETECTION__BUILDER_HPP_
#define YOLO_MSGS__MSG__DETAIL__DETECTION__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "yolo_msgs/msg/detail/detection__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace yolo_msgs
{

namespace msg
{

namespace builder
{

class Init_Detection_y2
{
public:
  explicit Init_Detection_y2(::yolo_msgs::msg::Detection & msg)
  : msg_(msg)
  {}
  ::yolo_msgs::msg::Detection y2(::yolo_msgs::msg::Detection::_y2_type arg)
  {
    msg_.y2 = std::move(arg);
    return std::move(msg_);
  }

private:
  ::yolo_msgs::msg::Detection msg_;
};

class Init_Detection_x2
{
public:
  explicit Init_Detection_x2(::yolo_msgs::msg::Detection & msg)
  : msg_(msg)
  {}
  Init_Detection_y2 x2(::yolo_msgs::msg::Detection::_x2_type arg)
  {
    msg_.x2 = std::move(arg);
    return Init_Detection_y2(msg_);
  }

private:
  ::yolo_msgs::msg::Detection msg_;
};

class Init_Detection_y1
{
public:
  explicit Init_Detection_y1(::yolo_msgs::msg::Detection & msg)
  : msg_(msg)
  {}
  Init_Detection_x2 y1(::yolo_msgs::msg::Detection::_y1_type arg)
  {
    msg_.y1 = std::move(arg);
    return Init_Detection_x2(msg_);
  }

private:
  ::yolo_msgs::msg::Detection msg_;
};

class Init_Detection_x1
{
public:
  explicit Init_Detection_x1(::yolo_msgs::msg::Detection & msg)
  : msg_(msg)
  {}
  Init_Detection_y1 x1(::yolo_msgs::msg::Detection::_x1_type arg)
  {
    msg_.x1 = std::move(arg);
    return Init_Detection_y1(msg_);
  }

private:
  ::yolo_msgs::msg::Detection msg_;
};

class Init_Detection_confidence
{
public:
  explicit Init_Detection_confidence(::yolo_msgs::msg::Detection & msg)
  : msg_(msg)
  {}
  Init_Detection_x1 confidence(::yolo_msgs::msg::Detection::_confidence_type arg)
  {
    msg_.confidence = std::move(arg);
    return Init_Detection_x1(msg_);
  }

private:
  ::yolo_msgs::msg::Detection msg_;
};

class Init_Detection_label
{
public:
  Init_Detection_label()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_Detection_confidence label(::yolo_msgs::msg::Detection::_label_type arg)
  {
    msg_.label = std::move(arg);
    return Init_Detection_confidence(msg_);
  }

private:
  ::yolo_msgs::msg::Detection msg_;
};

}  // namespace builder

}  // namespace msg

template<typename MessageType>
auto build();

template<>
inline
auto build<::yolo_msgs::msg::Detection>()
{
  return yolo_msgs::msg::builder::Init_Detection_label();
}

}  // namespace yolo_msgs

#endif  // YOLO_MSGS__MSG__DETAIL__DETECTION__BUILDER_HPP_
