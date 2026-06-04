import json

import cv2
import numpy as np
import rclpy
from cv_bridge import CvBridge
from rclpy.executors import ExternalShutdownException
from rclpy.node import Node
from sensor_msgs.msg import Image
from std_msgs.msg import String


class Topic2ObjectCounter(Node):
    """Simple camera-based object counting demo for the official atlas robot."""

    def __init__(self):
        super().__init__('topic2_object_counter')

        self.bridge = CvBridge()
        self.create_subscription(
            Image,
            '/atlas/rgbd_camera/image',
            self.image_cb,
            10,
        )

        self.status_pub = self.create_publisher(String, '/counting/status', 10)
        self.debug_pub = self.create_publisher(
            Image,
            '/object_counter/debug_image',
            10,
        )

        self.get_logger().info(
            'Topic 2 object counter started: '
            '/atlas/rgbd_camera/image -> /counting/status + '
            '/object_counter/debug_image'
        )

    @staticmethod
    def count_blobs(mask, min_area):
        contours, _ = cv2.findContours(
            mask,
            cv2.RETR_EXTERNAL,
            cv2.CHAIN_APPROX_SIMPLE,
        )
        return [
            contour
            for contour in contours
            if cv2.contourArea(contour) >= min_area
        ]

    def image_cb(self, msg):
        try:
            frame = self.bridge.imgmsg_to_cv2(msg, desired_encoding='bgr8')
        except Exception as exc:
            self.get_logger().warn(f'Could not convert camera image: {exc}')
            return

        debug = frame.copy()
        hsv = cv2.cvtColor(frame, cv2.COLOR_BGR2HSV)

        # Orange regions in the poster/sector imagery.
        orange_mask = cv2.inRange(
            hsv,
            np.array([5, 80, 80]),
            np.array([25, 255, 255]),
        )
        orange_mask = cv2.morphologyEx(
            orange_mask,
            cv2.MORPH_OPEN,
            np.ones((5, 5), np.uint8),
        )
        orange_contours = self.count_blobs(orange_mask, min_area=180)

        # Tree/trunk heuristic: dark, tall, narrow regions.
        gray = cv2.cvtColor(frame, cv2.COLOR_BGR2GRAY)
        dark_mask = cv2.inRange(gray, 0, 80)
        dark_mask = cv2.morphologyEx(
            dark_mask,
            cv2.MORPH_OPEN,
            np.ones((3, 7), np.uint8),
        )
        trunk_contours = []
        contours, _ = cv2.findContours(
            dark_mask,
            cv2.RETR_EXTERNAL,
            cv2.CHAIN_APPROX_SIMPLE,
        )
        for contour in contours:
            x, y, w, h = cv2.boundingRect(contour)
            area = cv2.contourArea(contour)
            if area > 120 and h > w * 1.5 and h > 25:
                trunk_contours.append(contour)

        # Vehicle heuristic: saturated rectangular image regions.
        saturation = hsv[:, :, 1]
        vehicle_mask = cv2.inRange(saturation, 70, 255)
        vehicle_mask = cv2.morphologyEx(
            vehicle_mask,
            cv2.MORPH_OPEN,
            np.ones((5, 5), np.uint8),
        )
        vehicle_contours = []
        contours, _ = cv2.findContours(
            vehicle_mask,
            cv2.RETR_EXTERNAL,
            cv2.CHAIN_APPROX_SIMPLE,
        )
        for contour in contours:
            x, y, w, h = cv2.boundingRect(contour)
            area = cv2.contourArea(contour)
            if area > 300 and w > 25 and h > 15:
                vehicle_contours.append(contour)

        counts = {
            'orange': len(orange_contours),
            'tree': len(trunk_contours),
            'vehicle': len(vehicle_contours),
        }

        self.draw_contours(debug, orange_contours, 'orange', (0, 165, 255))
        self.draw_contours(debug, trunk_contours, 'tree/trunk', (0, 255, 0))
        self.draw_contours(debug, vehicle_contours, 'vehicle', (255, 0, 0))

        summary = (
            f"oranges={counts['orange']} "
            f"trees={counts['tree']} "
            f"vehicles={counts['vehicle']}"
        )
        cv2.putText(
            debug,
            summary,
            (15, 30),
            cv2.FONT_HERSHEY_SIMPLEX,
            0.8,
            (255, 255, 255),
            2,
        )

        status = String()
        status.data = json.dumps(counts)
        self.status_pub.publish(status)

        try:
            self.debug_pub.publish(
                self.bridge.cv2_to_imgmsg(debug, encoding='bgr8')
            )
        except Exception as exc:
            self.get_logger().warn(f'Could not publish debug image: {exc}')

    @staticmethod
    def draw_contours(image, contours, label, color):
        for contour in contours:
            x, y, w, h = cv2.boundingRect(contour)
            cv2.rectangle(image, (x, y), (x + w, y + h), color, 2)
            cv2.putText(
                image,
                label,
                (x, max(y - 5, 15)),
                cv2.FONT_HERSHEY_SIMPLEX,
                0.5,
                color,
                2,
            )


def main(args=None):
    rclpy.init(args=args)
    node = Topic2ObjectCounter()
    try:
        rclpy.spin(node)
    except (KeyboardInterrupt, ExternalShutdownException):
        pass
    node.destroy_node()
    if rclpy.ok():
        rclpy.shutdown()


if __name__ == '__main__':
    main()
