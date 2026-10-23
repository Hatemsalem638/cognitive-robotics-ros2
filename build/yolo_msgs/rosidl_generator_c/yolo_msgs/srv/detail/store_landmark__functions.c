// generated from rosidl_generator_c/resource/idl__functions.c.em
// with input from yolo_msgs:srv/StoreLandmark.idl
// generated code does not contain a copyright notice
#include "yolo_msgs/srv/detail/store_landmark__functions.h"

#include <assert.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>

#include "rcutils/allocator.h"

// Include directives for member types
// Member `label`
#include "rosidl_runtime_c/string_functions.h"

bool
yolo_msgs__srv__StoreLandmark_Request__init(yolo_msgs__srv__StoreLandmark_Request * msg)
{
  if (!msg) {
    return false;
  }
  // label
  if (!rosidl_runtime_c__String__init(&msg->label)) {
    yolo_msgs__srv__StoreLandmark_Request__fini(msg);
    return false;
  }
  // x
  // y
  // z
  // confidence
  return true;
}

void
yolo_msgs__srv__StoreLandmark_Request__fini(yolo_msgs__srv__StoreLandmark_Request * msg)
{
  if (!msg) {
    return;
  }
  // label
  rosidl_runtime_c__String__fini(&msg->label);
  // x
  // y
  // z
  // confidence
}

bool
yolo_msgs__srv__StoreLandmark_Request__are_equal(const yolo_msgs__srv__StoreLandmark_Request * lhs, const yolo_msgs__srv__StoreLandmark_Request * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  // label
  if (!rosidl_runtime_c__String__are_equal(
      &(lhs->label), &(rhs->label)))
  {
    return false;
  }
  // x
  if (lhs->x != rhs->x) {
    return false;
  }
  // y
  if (lhs->y != rhs->y) {
    return false;
  }
  // z
  if (lhs->z != rhs->z) {
    return false;
  }
  // confidence
  if (lhs->confidence != rhs->confidence) {
    return false;
  }
  return true;
}

bool
yolo_msgs__srv__StoreLandmark_Request__copy(
  const yolo_msgs__srv__StoreLandmark_Request * input,
  yolo_msgs__srv__StoreLandmark_Request * output)
{
  if (!input || !output) {
    return false;
  }
  // label
  if (!rosidl_runtime_c__String__copy(
      &(input->label), &(output->label)))
  {
    return false;
  }
  // x
  output->x = input->x;
  // y
  output->y = input->y;
  // z
  output->z = input->z;
  // confidence
  output->confidence = input->confidence;
  return true;
}

yolo_msgs__srv__StoreLandmark_Request *
yolo_msgs__srv__StoreLandmark_Request__create()
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  yolo_msgs__srv__StoreLandmark_Request * msg = (yolo_msgs__srv__StoreLandmark_Request *)allocator.allocate(sizeof(yolo_msgs__srv__StoreLandmark_Request), allocator.state);
  if (!msg) {
    return NULL;
  }
  memset(msg, 0, sizeof(yolo_msgs__srv__StoreLandmark_Request));
  bool success = yolo_msgs__srv__StoreLandmark_Request__init(msg);
  if (!success) {
    allocator.deallocate(msg, allocator.state);
    return NULL;
  }
  return msg;
}

void
yolo_msgs__srv__StoreLandmark_Request__destroy(yolo_msgs__srv__StoreLandmark_Request * msg)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (msg) {
    yolo_msgs__srv__StoreLandmark_Request__fini(msg);
  }
  allocator.deallocate(msg, allocator.state);
}


bool
yolo_msgs__srv__StoreLandmark_Request__Sequence__init(yolo_msgs__srv__StoreLandmark_Request__Sequence * array, size_t size)
{
  if (!array) {
    return false;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  yolo_msgs__srv__StoreLandmark_Request * data = NULL;

  if (size) {
    data = (yolo_msgs__srv__StoreLandmark_Request *)allocator.zero_allocate(size, sizeof(yolo_msgs__srv__StoreLandmark_Request), allocator.state);
    if (!data) {
      return false;
    }
    // initialize all array elements
    size_t i;
    for (i = 0; i < size; ++i) {
      bool success = yolo_msgs__srv__StoreLandmark_Request__init(&data[i]);
      if (!success) {
        break;
      }
    }
    if (i < size) {
      // if initialization failed finalize the already initialized array elements
      for (; i > 0; --i) {
        yolo_msgs__srv__StoreLandmark_Request__fini(&data[i - 1]);
      }
      allocator.deallocate(data, allocator.state);
      return false;
    }
  }
  array->data = data;
  array->size = size;
  array->capacity = size;
  return true;
}

void
yolo_msgs__srv__StoreLandmark_Request__Sequence__fini(yolo_msgs__srv__StoreLandmark_Request__Sequence * array)
{
  if (!array) {
    return;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();

  if (array->data) {
    // ensure that data and capacity values are consistent
    assert(array->capacity > 0);
    // finalize all array elements
    for (size_t i = 0; i < array->capacity; ++i) {
      yolo_msgs__srv__StoreLandmark_Request__fini(&array->data[i]);
    }
    allocator.deallocate(array->data, allocator.state);
    array->data = NULL;
    array->size = 0;
    array->capacity = 0;
  } else {
    // ensure that data, size, and capacity values are consistent
    assert(0 == array->size);
    assert(0 == array->capacity);
  }
}

yolo_msgs__srv__StoreLandmark_Request__Sequence *
yolo_msgs__srv__StoreLandmark_Request__Sequence__create(size_t size)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  yolo_msgs__srv__StoreLandmark_Request__Sequence * array = (yolo_msgs__srv__StoreLandmark_Request__Sequence *)allocator.allocate(sizeof(yolo_msgs__srv__StoreLandmark_Request__Sequence), allocator.state);
  if (!array) {
    return NULL;
  }
  bool success = yolo_msgs__srv__StoreLandmark_Request__Sequence__init(array, size);
  if (!success) {
    allocator.deallocate(array, allocator.state);
    return NULL;
  }
  return array;
}

void
yolo_msgs__srv__StoreLandmark_Request__Sequence__destroy(yolo_msgs__srv__StoreLandmark_Request__Sequence * array)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (array) {
    yolo_msgs__srv__StoreLandmark_Request__Sequence__fini(array);
  }
  allocator.deallocate(array, allocator.state);
}

bool
yolo_msgs__srv__StoreLandmark_Request__Sequence__are_equal(const yolo_msgs__srv__StoreLandmark_Request__Sequence * lhs, const yolo_msgs__srv__StoreLandmark_Request__Sequence * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  if (lhs->size != rhs->size) {
    return false;
  }
  for (size_t i = 0; i < lhs->size; ++i) {
    if (!yolo_msgs__srv__StoreLandmark_Request__are_equal(&(lhs->data[i]), &(rhs->data[i]))) {
      return false;
    }
  }
  return true;
}

bool
yolo_msgs__srv__StoreLandmark_Request__Sequence__copy(
  const yolo_msgs__srv__StoreLandmark_Request__Sequence * input,
  yolo_msgs__srv__StoreLandmark_Request__Sequence * output)
{
  if (!input || !output) {
    return false;
  }
  if (output->capacity < input->size) {
    const size_t allocation_size =
      input->size * sizeof(yolo_msgs__srv__StoreLandmark_Request);
    rcutils_allocator_t allocator = rcutils_get_default_allocator();
    yolo_msgs__srv__StoreLandmark_Request * data =
      (yolo_msgs__srv__StoreLandmark_Request *)allocator.reallocate(
      output->data, allocation_size, allocator.state);
    if (!data) {
      return false;
    }
    // If reallocation succeeded, memory may or may not have been moved
    // to fulfill the allocation request, invalidating output->data.
    output->data = data;
    for (size_t i = output->capacity; i < input->size; ++i) {
      if (!yolo_msgs__srv__StoreLandmark_Request__init(&output->data[i])) {
        // If initialization of any new item fails, roll back
        // all previously initialized items. Existing items
        // in output are to be left unmodified.
        for (; i-- > output->capacity; ) {
          yolo_msgs__srv__StoreLandmark_Request__fini(&output->data[i]);
        }
        return false;
      }
    }
    output->capacity = input->size;
  }
  output->size = input->size;
  for (size_t i = 0; i < input->size; ++i) {
    if (!yolo_msgs__srv__StoreLandmark_Request__copy(
        &(input->data[i]), &(output->data[i])))
    {
      return false;
    }
  }
  return true;
}


// Include directives for member types
// Member `message`
// already included above
// #include "rosidl_runtime_c/string_functions.h"

bool
yolo_msgs__srv__StoreLandmark_Response__init(yolo_msgs__srv__StoreLandmark_Response * msg)
{
  if (!msg) {
    return false;
  }
  // success
  // message
  if (!rosidl_runtime_c__String__init(&msg->message)) {
    yolo_msgs__srv__StoreLandmark_Response__fini(msg);
    return false;
  }
  return true;
}

void
yolo_msgs__srv__StoreLandmark_Response__fini(yolo_msgs__srv__StoreLandmark_Response * msg)
{
  if (!msg) {
    return;
  }
  // success
  // message
  rosidl_runtime_c__String__fini(&msg->message);
}

bool
yolo_msgs__srv__StoreLandmark_Response__are_equal(const yolo_msgs__srv__StoreLandmark_Response * lhs, const yolo_msgs__srv__StoreLandmark_Response * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  // success
  if (lhs->success != rhs->success) {
    return false;
  }
  // message
  if (!rosidl_runtime_c__String__are_equal(
      &(lhs->message), &(rhs->message)))
  {
    return false;
  }
  return true;
}

bool
yolo_msgs__srv__StoreLandmark_Response__copy(
  const yolo_msgs__srv__StoreLandmark_Response * input,
  yolo_msgs__srv__StoreLandmark_Response * output)
{
  if (!input || !output) {
    return false;
  }
  // success
  output->success = input->success;
  // message
  if (!rosidl_runtime_c__String__copy(
      &(input->message), &(output->message)))
  {
    return false;
  }
  return true;
}

yolo_msgs__srv__StoreLandmark_Response *
yolo_msgs__srv__StoreLandmark_Response__create()
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  yolo_msgs__srv__StoreLandmark_Response * msg = (yolo_msgs__srv__StoreLandmark_Response *)allocator.allocate(sizeof(yolo_msgs__srv__StoreLandmark_Response), allocator.state);
  if (!msg) {
    return NULL;
  }
  memset(msg, 0, sizeof(yolo_msgs__srv__StoreLandmark_Response));
  bool success = yolo_msgs__srv__StoreLandmark_Response__init(msg);
  if (!success) {
    allocator.deallocate(msg, allocator.state);
    return NULL;
  }
  return msg;
}

void
yolo_msgs__srv__StoreLandmark_Response__destroy(yolo_msgs__srv__StoreLandmark_Response * msg)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (msg) {
    yolo_msgs__srv__StoreLandmark_Response__fini(msg);
  }
  allocator.deallocate(msg, allocator.state);
}


bool
yolo_msgs__srv__StoreLandmark_Response__Sequence__init(yolo_msgs__srv__StoreLandmark_Response__Sequence * array, size_t size)
{
  if (!array) {
    return false;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  yolo_msgs__srv__StoreLandmark_Response * data = NULL;

  if (size) {
    data = (yolo_msgs__srv__StoreLandmark_Response *)allocator.zero_allocate(size, sizeof(yolo_msgs__srv__StoreLandmark_Response), allocator.state);
    if (!data) {
      return false;
    }
    // initialize all array elements
    size_t i;
    for (i = 0; i < size; ++i) {
      bool success = yolo_msgs__srv__StoreLandmark_Response__init(&data[i]);
      if (!success) {
        break;
      }
    }
    if (i < size) {
      // if initialization failed finalize the already initialized array elements
      for (; i > 0; --i) {
        yolo_msgs__srv__StoreLandmark_Response__fini(&data[i - 1]);
      }
      allocator.deallocate(data, allocator.state);
      return false;
    }
  }
  array->data = data;
  array->size = size;
  array->capacity = size;
  return true;
}

void
yolo_msgs__srv__StoreLandmark_Response__Sequence__fini(yolo_msgs__srv__StoreLandmark_Response__Sequence * array)
{
  if (!array) {
    return;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();

  if (array->data) {
    // ensure that data and capacity values are consistent
    assert(array->capacity > 0);
    // finalize all array elements
    for (size_t i = 0; i < array->capacity; ++i) {
      yolo_msgs__srv__StoreLandmark_Response__fini(&array->data[i]);
    }
    allocator.deallocate(array->data, allocator.state);
    array->data = NULL;
    array->size = 0;
    array->capacity = 0;
  } else {
    // ensure that data, size, and capacity values are consistent
    assert(0 == array->size);
    assert(0 == array->capacity);
  }
}

yolo_msgs__srv__StoreLandmark_Response__Sequence *
yolo_msgs__srv__StoreLandmark_Response__Sequence__create(size_t size)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  yolo_msgs__srv__StoreLandmark_Response__Sequence * array = (yolo_msgs__srv__StoreLandmark_Response__Sequence *)allocator.allocate(sizeof(yolo_msgs__srv__StoreLandmark_Response__Sequence), allocator.state);
  if (!array) {
    return NULL;
  }
  bool success = yolo_msgs__srv__StoreLandmark_Response__Sequence__init(array, size);
  if (!success) {
    allocator.deallocate(array, allocator.state);
    return NULL;
  }
  return array;
}

void
yolo_msgs__srv__StoreLandmark_Response__Sequence__destroy(yolo_msgs__srv__StoreLandmark_Response__Sequence * array)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (array) {
    yolo_msgs__srv__StoreLandmark_Response__Sequence__fini(array);
  }
  allocator.deallocate(array, allocator.state);
}

bool
yolo_msgs__srv__StoreLandmark_Response__Sequence__are_equal(const yolo_msgs__srv__StoreLandmark_Response__Sequence * lhs, const yolo_msgs__srv__StoreLandmark_Response__Sequence * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  if (lhs->size != rhs->size) {
    return false;
  }
  for (size_t i = 0; i < lhs->size; ++i) {
    if (!yolo_msgs__srv__StoreLandmark_Response__are_equal(&(lhs->data[i]), &(rhs->data[i]))) {
      return false;
    }
  }
  return true;
}

bool
yolo_msgs__srv__StoreLandmark_Response__Sequence__copy(
  const yolo_msgs__srv__StoreLandmark_Response__Sequence * input,
  yolo_msgs__srv__StoreLandmark_Response__Sequence * output)
{
  if (!input || !output) {
    return false;
  }
  if (output->capacity < input->size) {
    const size_t allocation_size =
      input->size * sizeof(yolo_msgs__srv__StoreLandmark_Response);
    rcutils_allocator_t allocator = rcutils_get_default_allocator();
    yolo_msgs__srv__StoreLandmark_Response * data =
      (yolo_msgs__srv__StoreLandmark_Response *)allocator.reallocate(
      output->data, allocation_size, allocator.state);
    if (!data) {
      return false;
    }
    // If reallocation succeeded, memory may or may not have been moved
    // to fulfill the allocation request, invalidating output->data.
    output->data = data;
    for (size_t i = output->capacity; i < input->size; ++i) {
      if (!yolo_msgs__srv__StoreLandmark_Response__init(&output->data[i])) {
        // If initialization of any new item fails, roll back
        // all previously initialized items. Existing items
        // in output are to be left unmodified.
        for (; i-- > output->capacity; ) {
          yolo_msgs__srv__StoreLandmark_Response__fini(&output->data[i]);
        }
        return false;
      }
    }
    output->capacity = input->size;
  }
  output->size = input->size;
  for (size_t i = 0; i < input->size; ++i) {
    if (!yolo_msgs__srv__StoreLandmark_Response__copy(
        &(input->data[i]), &(output->data[i])))
    {
      return false;
    }
  }
  return true;
}
