import math
from dataclasses import dataclass

import numpy as np
import rclpy
from geometry_msgs.msg import Twist
from nav_msgs.msg import Odometry
from rclpy.executors import ExternalShutdownException
from rclpy.node import Node
from rclpy.qos import HistoryPolicy, QoSProfile, ReliabilityPolicy
from sensor_msgs.msg import PointCloud2
from sensor_msgs_py import point_cloud2


@dataclass(frozen=True)
class RouteSegment:
    action: str
    value: float = 0.0


DEFAULT_ROUTE = [
    RouteSegment('DRIVE_FORWARD', 1.0),
    RouteSegment('TURN_LEFT', 90.0),
    RouteSegment('DRIVE_FORWARD', 1.2),
    RouteSegment('TURN_RIGHT', 90.0),
    RouteSegment('DRIVE_FORWARD', 1.0),
    RouteSegment('PAUSE', 1.0),
    RouteSegment('DRIVE_FORWARD', 0.8),
    RouteSegment('STOP', 0.0),
]


class Topic2EnhancedNavigation(Node):
    """Route-based navigation with RGB-D obstacle safety for the atlas robot."""

    def __init__(self):
        super().__init__('topic2_enhanced_navigation')

        self.declare_parameter('forward_speed', 0.10)
        self.declare_parameter('turn_speed', 0.30)
        self.declare_parameter('obstacle_turn_speed', 0.35)
        self.declare_parameter('front_stop_distance', 0.45)
        self.declare_parameter('front_clear_distance', 0.80)
        self.declare_parameter('control_rate_hz', 10.0)
        self.declare_parameter('front_half_width', 0.35)
        self.declare_parameter('vertical_min', -0.15)
        self.declare_parameter('vertical_max', 0.80)
        self.declare_parameter('max_front_range', 3.0)
        self.declare_parameter('distance_tolerance', 0.05)
        self.declare_parameter('turn_tolerance_deg', 3.0)

        self.forward_speed = float(self.get_parameter('forward_speed').value)
        self.turn_speed = float(self.get_parameter('turn_speed').value)
        self.obstacle_turn_speed = float(
            self.get_parameter('obstacle_turn_speed').value
        )
        self.front_stop_distance = float(
            self.get_parameter('front_stop_distance').value
        )
        self.front_clear_distance = float(
            self.get_parameter('front_clear_distance').value
        )
        self.control_rate_hz = float(self.get_parameter('control_rate_hz').value)
        self.front_half_width = float(self.get_parameter('front_half_width').value)
        self.vertical_min = float(self.get_parameter('vertical_min').value)
        self.vertical_max = float(self.get_parameter('vertical_max').value)
        self.max_front_range = float(self.get_parameter('max_front_range').value)
        self.distance_tolerance = float(
            self.get_parameter('distance_tolerance').value
        )
        self.turn_tolerance = math.radians(
            float(self.get_parameter('turn_tolerance_deg').value)
        )

        self.route = DEFAULT_ROUTE
        self.segment_index = 0
        self.segment_started = False
        self.segment_start_x = 0.0
        self.segment_start_y = 0.0
        self.segment_start_yaw = 0.0
        self.segment_start_time = None
        self.obstacle_override_active = False

        self.x = None
        self.y = None
        self.yaw = None
        self.front_distance = math.inf

        sensor_qos = QoSProfile(
            reliability=ReliabilityPolicy.BEST_EFFORT,
            history=HistoryPolicy.KEEP_LAST,
            depth=1,
        )
        reliable_qos = QoSProfile(
            reliability=ReliabilityPolicy.RELIABLE,
            history=HistoryPolicy.KEEP_LAST,
            depth=10,
        )

        self.cmd_pub = self.create_publisher(Twist, '/atlas/cmd_vel', reliable_qos)
        self.odom_sub = self.create_subscription(
            Odometry,
            '/atlas/odom_ground_truth',
            self.odom_callback,
            reliable_qos,
        )
        self.cloud_sub = self.create_subscription(
            PointCloud2,
            '/atlas/rgbd_camera/points',
            self.point_cloud_callback,
            sensor_qos,
        )

        timer_period = 1.0 / max(self.control_rate_hz, 1.0)
        self.control_timer = self.create_timer(timer_period, self.control_loop)

        self.get_logger().info(
            'Topic 2 enhanced navigation started: '
            '/atlas/odom_ground_truth + /atlas/rgbd_camera/points '
            '-> /atlas/cmd_vel'
        )
        self.get_logger().info(
            'Camera convention: +x forward, y side, z vertical'
        )
        self.get_logger().info(
            f'Route contains {len(self.route)} segments: '
            + ', '.join(f'{s.action}:{s.value}' for s in self.route)
        )

    def odom_callback(self, msg):
        pose = msg.pose.pose
        self.x = float(pose.position.x)
        self.y = float(pose.position.y)
        self.yaw = self.yaw_from_quaternion(pose.orientation)

    def point_cloud_callback(self, msg):
        try:
            points = self.read_xyz_points(msg)
            self.front_distance = self.compute_front_distance(points)
        except Exception as exc:
            self.get_logger().warning(
                f'Point cloud parsing failed: {exc}',
                throttle_duration_sec=2.0,
            )
            self.front_distance = math.inf

    def control_loop(self):
        if self.x is None or self.y is None or self.yaw is None:
            self.publish_command(0.0, 0.0)
            self.get_logger().info(
                'Waiting for /atlas/odom_ground_truth',
                throttle_duration_sec=2.0,
            )
            return

        if self.segment_index >= len(self.route):
            self.publish_command(0.0, 0.0)
            return

        segment = self.route[self.segment_index]
        if not self.segment_started:
            self.start_segment(segment)

        if segment.action == 'DRIVE_FORWARD':
            linear_x, angular_z, progress = self.drive_forward_command(segment)
        elif segment.action in ('TURN_LEFT', 'TURN_RIGHT'):
            linear_x, angular_z, progress = self.turn_command(segment)
        elif segment.action == 'PAUSE':
            linear_x, angular_z, progress = self.pause_command(segment)
        elif segment.action == 'STOP':
            linear_x, angular_z, progress = 0.0, 0.0, 'complete'
            self.publish_command(linear_x, angular_z)
            self.get_logger().info('STOP segment reached; navigation complete')
            self.segment_index = len(self.route)
            return
        else:
            self.get_logger().warning(
                f'Unknown route action {segment.action}; skipping segment'
            )
            self.advance_segment()
            return

        self.publish_command(linear_x, angular_z)
        self.log_status(segment, progress, linear_x, angular_z)

    def start_segment(self, segment):
        self.segment_start_x = self.x
        self.segment_start_y = self.y
        self.segment_start_yaw = self.yaw
        self.segment_start_time = self.get_clock().now()
        self.segment_started = True
        self.obstacle_override_active = False

        self.get_logger().info(
            f'Starting segment {self.segment_index}: '
            f'{segment.action} {segment.value}'
        )

    def drive_forward_command(self, segment):
        distance = math.hypot(
            self.x - self.segment_start_x,
            self.y - self.segment_start_y,
        )

        if distance + self.distance_tolerance >= segment.value:
            self.get_logger().info(
                f'Completed segment {self.segment_index}: '
                f'DRIVE_FORWARD travelled={distance:.2f}m'
            )
            self.advance_segment()
            return 0.0, 0.0, distance

        if self.should_enter_obstacle_override():
            self.obstacle_override_active = True
            self.get_logger().warning(
                f'Obstacle safety override active: '
                f'front={self.format_distance(self.front_distance)}m',
                throttle_duration_sec=1.0,
            )

        if self.obstacle_override_active:
            if self.front_distance > self.front_clear_distance:
                self.obstacle_override_active = False
                self.get_logger().info(
                    f'Obstacle cleared: '
                    f'front={self.format_distance(self.front_distance)}m; '
                    'resuming route segment'
                )
            else:
                return 0.0, self.obstacle_turn_speed, distance

        return self.forward_speed, 0.0, distance

    def turn_command(self, segment):
        target = math.radians(abs(segment.value))
        direction = 1.0 if segment.action == 'TURN_LEFT' else -1.0
        yaw_delta = self.normalize_angle(self.yaw - self.segment_start_yaw)
        turned = direction * yaw_delta
        remaining = target - turned

        if remaining <= self.turn_tolerance:
            self.get_logger().info(
                f'Completed segment {self.segment_index}: '
                f'{segment.action} turned={math.degrees(turned):.1f}deg'
            )
            self.advance_segment()
            return 0.0, 0.0, turned

        return 0.0, direction * self.turn_speed, turned

    def pause_command(self, segment):
        elapsed = (
            self.get_clock().now() - self.segment_start_time
        ).nanoseconds / 1e9
        if elapsed >= segment.value:
            self.get_logger().info(
                f'Completed segment {self.segment_index}: '
                f'PAUSE elapsed={elapsed:.1f}s'
            )
            self.advance_segment()
        return 0.0, 0.0, elapsed

    def should_enter_obstacle_override(self):
        return (
            math.isfinite(self.front_distance) and
            self.front_distance < self.front_stop_distance
        )

    def advance_segment(self):
        self.segment_index += 1
        self.segment_started = False
        self.obstacle_override_active = False

    def publish_command(self, linear_x, angular_z):
        twist = Twist()
        twist.linear.x = max(0.0, float(linear_x))
        twist.angular.z = float(
            np.clip(
                angular_z,
                -max(self.turn_speed, self.obstacle_turn_speed),
                max(self.turn_speed, self.obstacle_turn_speed),
            )
        )
        self.cmd_pub.publish(twist)

    def log_status(self, segment, progress, linear_x, angular_z):
        if isinstance(progress, (float, int)):
            progress_text = f'{progress:.2f}'
        else:
            progress_text = str(progress)

        self.get_logger().info(
            f'segment={self.segment_index}/{len(self.route) - 1} '
            f'state={segment.action} '
            f'progress={progress_text} '
            f'front={self.format_distance(self.front_distance)}m '
            f'cmd=({max(0.0, linear_x):.2f}, {angular_z:.2f}) '
            f'obstacle_override={self.obstacle_override_active}',
            throttle_duration_sec=1.0,
        )

    def compute_front_distance(self, points):
        if points.shape[0] == 0:
            return math.inf

        finite_mask = np.isfinite(points).all(axis=1)
        points = points[finite_mask]
        if points.shape[0] == 0:
            return math.inf

        x_forward = points[:, 0]
        y_side = points[:, 1]
        z_vertical = points[:, 2]
        front_mask = (
            (x_forward > 0.05) &
            (x_forward < self.max_front_range) &
            (np.abs(y_side) <= self.front_half_width) &
            (z_vertical > self.vertical_min) &
            (z_vertical < self.vertical_max)
        )
        front_points = x_forward[front_mask]
        if front_points.size == 0:
            return math.inf
        return float(np.min(front_points))

    def read_xyz_points(self, msg):
        raw_points = point_cloud2.read_points(
            msg,
            field_names=('x', 'y', 'z'),
            skip_nans=True,
        )

        if isinstance(raw_points, np.ndarray) and raw_points.dtype.names:
            xs = raw_points['x'].astype(np.float64)
            ys = raw_points['y'].astype(np.float64)
            zs = raw_points['z'].astype(np.float64)
            return np.column_stack((xs, ys, zs))

        points = np.asarray(list(raw_points), dtype=np.float64)
        if points.size == 0:
            return np.empty((0, 3), dtype=np.float64)
        return points.reshape((-1, 3))

    def yaw_from_quaternion(self, quat):
        return math.atan2(
            2.0 * (quat.w * quat.z + quat.x * quat.y),
            1.0 - 2.0 * (quat.y * quat.y + quat.z * quat.z),
        )

    def normalize_angle(self, angle):
        while angle > math.pi:
            angle -= 2.0 * math.pi
        while angle < -math.pi:
            angle += 2.0 * math.pi
        return angle

    def format_distance(self, distance):
        if math.isfinite(distance):
            return f'{distance:.2f}'
        return 'inf'

    def publish_stop(self):
        self.publish_command(0.0, 0.0)


def main(args=None):
    rclpy.init(args=args)
    node = Topic2EnhancedNavigation()

    try:
        rclpy.spin(node)
    except (KeyboardInterrupt, ExternalShutdownException):
        pass
    finally:
        if rclpy.ok():
            node.publish_stop()
        node.destroy_node()
        if rclpy.ok():
            rclpy.shutdown()


if __name__ == '__main__':
    main()
