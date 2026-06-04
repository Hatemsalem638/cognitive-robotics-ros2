"""
Wall-Following Navigation Node.

This node implements reactive wall-following navigation for the JetBot maze runner.
It uses depth sensor data and a PID controller to maintain a target distance from walls.

The robot follows walls by:
1. Reading depth/lidar data from the RealSense camera
2. Filtering noisy readings with a Kalman filter
3. Computing the desired distance error from the target wall distance
4. Applying PID control to generate twist commands
5. Publishing velocity commands for robot movement

Topics:
    Subscriptions:
        /camera/depth/image_raw: Depth image from RealSense (sensor_msgs/Image)
        /scan: LaserScan for backup (sensor_msgs/LaserScan)
    Publications:
        /cmd_vel: Twist commands for robot motion (geometry_msgs/Twist)
"""

import rclpy
from rclpy.node import Node
from rclpy.qos import QoSProfile, ReliabilityPolicy, HistoryPolicy

import numpy as np
from geometry_msgs.msg import Twist
from sensor_msgs.msg import Image, LaserScan

try:
    from cv_bridge import CvBridge
    HAVE_CV_BRIDGE = True
except ImportError:
    HAVE_CV_BRIDGE = False

from .kalman_filter import KalmanFilter1D


class WallFollowerNode(Node):
    """
    ROS2 node implementing reactive wall-following using PID control.
    
    The wall-follower uses a simple but robust strategy:
    - Divide the depth image into three regions: left, center, right
    - Compute average depth in each region
    - If left side is closer to target distance, steer right (and vice versa)
    - Use center depth to avoid obstacles ahead
    """
    
    def __init__(self):
        super().__init__('wall_follower')
        
        
        # Reliable QoS for velocity commands.
        # Gazebo diff_drive expects reliable/default QoS on /cmd_vel.
        self.cmd_vel_qos = QoSProfile(
            reliability=ReliabilityPolicy.RELIABLE,
            history=HistoryPolicy.KEEP_LAST,
            depth=10
        )

        # Best-effort QoS for sensor streams such as /scan and depth images.
        self.sensor_qos = QoSProfile(
            reliability=ReliabilityPolicy.BEST_EFFORT,
            history=HistoryPolicy.KEEP_LAST,
            depth=10
        )

# Declare parameters
        self.declare_parameter('target_wall_distance', 0.5)
        self.declare_parameter('linear_speed', 0.2)
        self.declare_parameter('pid_kp', 0.8)
        self.declare_parameter('pid_ki', 0.0)
        self.declare_parameter('pid_kd', 0.2)
        self.declare_parameter('angular_max', 1.0)
        self.declare_parameter('depth_valid_range', [0.1, 3.0])
        self.declare_parameter('use_kalman_filter', True)
        
        # Get parameters
        self.target_wall_distance = self.get_parameter('target_wall_distance').value
        self.linear_speed = self.get_parameter('linear_speed').value
        self.pid_kp = self.get_parameter('pid_kp').value
        self.pid_ki = self.get_parameter('pid_ki').value
        self.pid_kd = self.get_parameter('pid_kd').value
        self.angular_max = self.get_parameter('angular_max').value
        depth_range = self.get_parameter('depth_valid_range').value
        self.depth_min = depth_range[0]
        self.depth_max = depth_range[1]
        self.use_kalman = self.get_parameter('use_kalman_filter').value
        
        # QoS profile for sensor data (best effort, low latency)
        qos = QoSProfile(
            reliability=ReliabilityPolicy.BEST_EFFORT,
            history=HistoryPolicy.KEEP_LAST,
            depth=1
        )
        
        # Publishers and subscribers
        self.twist_pub = self.create_publisher(
            Twist,
            '/cmd_vel',
            self.cmd_vel_qos
        )
        
        # Try depth image first (RealSense)
        if HAVE_CV_BRIDGE:
            self.depth_sub = self.create_subscription(
                Image,
                '/camera/depth/image_raw',
                self.depth_callback,
                qos_profile=qos
            )
            self.bridge = CvBridge()
        
        # LaserScan fallback
        self.scan_sub = self.create_subscription(
            LaserScan,
            '/scan',
            self.scan_callback,
            qos_profile=qos
        )
        
        # PID controller state
        self.prev_error = 0.0
        self.integral_error = 0.0
        self.invalid_scan_warning_period_ns = 5_000_000_000
        self.last_invalid_scan_warning = {}
        
        # Kalman filters for noise reduction
        self.kalman_left = KalmanFilter1D(process_noise=0.01, measurement_noise=0.05)
        self.kalman_right = KalmanFilter1D(process_noise=0.01, measurement_noise=0.05)
        self.kalman_center = KalmanFilter1D(process_noise=0.01, measurement_noise=0.05)
        
        self.get_logger().info(
            f'Wall-Follower initialized. Target: {self.target_wall_distance}m, '
            f'Speed: {self.linear_speed}m/s, Kalman: {self.use_kalman}'
        )
    
    def depth_callback(self, msg):
        """Process depth image from RealSense camera."""
        if not HAVE_CV_BRIDGE:
            return
            
        try:
            import cv2
            depth_image = self.bridge.imgmsg_to_cv2(msg, desired_encoding='32FC1')
            
            height, width = depth_image.shape
            
            # Mask invalid depth
            depth_valid = np.where(
                (depth_image > self.depth_min) & (depth_image < self.depth_max),
                depth_image,
                np.nan
            )
            
            # Left, center, right regions
            third = width // 3
            depth_left = np.nanmean(depth_valid[:, :third])
            depth_center = np.nanmean(depth_valid[:, third:2*third])
            depth_right = np.nanmean(depth_valid[:, 2*third:])
            
            # Handle NaN
            if np.isnan(depth_left):
                depth_left = self.target_wall_distance
            if np.isnan(depth_center):
                depth_center = self.target_wall_distance
            if np.isnan(depth_right):
                depth_right = self.target_wall_distance
            
            # Kalman filter
            if self.use_kalman:
                depth_left = self.kalman_left.filter_measurement(depth_left)
                depth_center = self.kalman_center.filter_measurement(depth_center)
                depth_right = self.kalman_right.filter_measurement(depth_right)
            
            self.compute_control(depth_left, depth_center, depth_right)
            
        except Exception as e:
            self.get_logger().error(f'Depth callback error: {e}')
    
    def scan_callback(self, msg):
        """Process LaserScan data."""
        try:
            ranges = np.array(msg.ranges, dtype=float)

            scan_min = msg.range_min if np.isfinite(msg.range_min) else self.depth_min
            scan_max = msg.range_max if np.isfinite(msg.range_max) else self.depth_max
            valid_min = max(self.depth_min, scan_min)
            valid_max = min(self.depth_max, scan_max)
            fallback_range = scan_max if scan_max > valid_min else self.depth_max

            valid_mask = (
                np.isfinite(ranges) &
                (ranges > valid_min) &
                (ranges < valid_max)
            )
            ranges = np.where(valid_mask, ranges, np.nan)

            # Divide into sectors
            n = len(ranges)
            third = n // 3

            left_sector = ranges[:third]
            center_sector = ranges[third:2*third]
            right_sector = ranges[2*third:]

            left_valid = np.any(np.isfinite(left_sector))
            center_valid = np.any(np.isfinite(center_sector))
            right_valid = np.any(np.isfinite(right_sector))

            self.warn_invalid_sector('left', left_valid)
            self.warn_invalid_sector('front', center_valid)
            self.warn_invalid_sector('right', right_valid)

            depth_left = self.safe_sector_mean(left_sector, fallback_range)
            depth_center = self.safe_sector_mean(center_sector, fallback_range)
            depth_right = self.safe_sector_mean(right_sector, fallback_range)

            # Kalman filter
            if self.use_kalman:
                depth_left = self.kalman_left.filter_measurement(depth_left)
                if center_valid:
                    depth_center = self.kalman_center.filter_measurement(depth_center)
                depth_right = self.kalman_right.filter_measurement(depth_right)

            self.compute_control(
                depth_left,
                depth_center,
                depth_right,
                front_valid=center_valid,
            )
            
        except Exception as e:
            self.get_logger().error(f'Scan callback error: {e}')

    def safe_sector_mean(self, sector, fallback):
        """Return the mean of valid sector values, or fallback if none exist."""
        valid_values = sector[np.isfinite(sector)]
        if valid_values.size == 0:
            return fallback
        return float(np.mean(valid_values))

    def warn_invalid_sector(self, sector_name, is_valid):
        if is_valid:
            return

        now = self.get_clock().now().nanoseconds
        last_warning = self.last_invalid_scan_warning.get(sector_name, 0)
        if now - last_warning < self.invalid_scan_warning_period_ns:
            return

        self.last_invalid_scan_warning[sector_name] = now
        self.get_logger().warning(
            f'Invalid {sector_name} scan sector; using fallback distance'
        )

    def compute_control(self, depth_left, depth_center, depth_right, front_valid=True):
        """Compute PID-based wall-following control."""
        # Wall-following error (positive = right side too close, steer left)
        error = (depth_right - depth_left) / 2.0
        
        # Front obstacle avoidance
        if not front_valid:
            forward_speed = 0.0
            angular_velocity = self.turn_away_from_closest_side(depth_left, depth_right)
        elif depth_center < 0.35:
            forward_speed = 0.0
            angular_velocity = self.turn_away_from_closest_side(depth_left, depth_right)
        elif depth_center < self.target_wall_distance * 0.7:
            forward_speed = 0.0
            angular_velocity = None
        else:
            forward_speed = self.linear_speed
            angular_velocity = None

        if angular_velocity is None:
            # PID
            self.integral_error += error * 0.1
            self.integral_error = np.clip(self.integral_error, -1.0, 1.0)

            derivative = (error - self.prev_error) / 0.1
            self.prev_error = error

            angular_velocity = (
                self.pid_kp * error +
                self.pid_ki * self.integral_error +
                self.pid_kd * derivative
            )
            angular_velocity = np.clip(
                angular_velocity,
                -self.angular_max,
                self.angular_max,
            )
        
        # Publish
        twist = Twist()
        twist.linear.x = float(forward_speed)
        twist.angular.z = float(angular_velocity)
        self.twist_pub.publish(twist)

    def turn_away_from_closest_side(self, depth_left, depth_right):
        """Rotate away from the closest side obstacle at a controlled speed."""
        turn_speed = min(0.4, self.angular_max)
        if depth_left < depth_right:
            return -turn_speed
        return turn_speed


def main(args=None):
    rclpy.init(args=args)
    node = WallFollowerNode()
    rclpy.spin(node)
    node.destroy_node()
    rclpy.shutdown()


if __name__ == '__main__':
    main()
