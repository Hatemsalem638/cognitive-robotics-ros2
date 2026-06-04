import math

import numpy as np
import rclpy
from geometry_msgs.msg import Twist
from rclpy.executors import ExternalShutdownException
from rclpy.node import Node
from rclpy.qos import HistoryPolicy, QoSProfile, ReliabilityPolicy
from sensor_msgs.msg import PointCloud2
from sensor_msgs_py import point_cloud2

from .kalman_filter import KalmanFilter1D


class Topic2DepthWallFollower(Node):
    """Depth-camera obstacle avoidance for the official Topic 2 atlas robot."""

    def __init__(self):
        super().__init__('topic2_depth_wall_follower')

        self.declare_parameter('linear_speed', 0.10)
        self.declare_parameter('turn_speed', 0.35)
        self.declare_parameter('default_turn_direction', 1.0)
        self.declare_parameter('front_stop_distance', 0.45)
        self.declare_parameter('slow_down_distance', 1.20)
        self.declare_parameter('turn_start_distance', 0.90)
        self.declare_parameter('side_target_distance', 0.55)
        self.declare_parameter('min_points_required', 50)
        self.declare_parameter('max_usable_distance', 3.0)
        self.declare_parameter('front_axis', 'x')
        self.declare_parameter('side_axis', 'y')
        self.declare_parameter('vertical_axis', 'z')
        self.declare_parameter('front_axis_sign', 1.0)
        self.declare_parameter('side_axis_sign', 1.0)
        self.declare_parameter('vertical_axis_sign', 1.0)
        self.declare_parameter('front_half_width', 0.35)
        self.declare_parameter('side_wall_min_abs_y', 0.15)
        self.declare_parameter('side_min_forward', 0.20)
        self.declare_parameter('side_max_forward', 2.00)
        self.declare_parameter('vertical_min', -0.15)
        self.declare_parameter('vertical_max', 0.80)
        self.declare_parameter('right_memory_duration', 1.5)
        self.declare_parameter('right_search_turn_speed', 0.12)
        self.declare_parameter('wall_follow_turn_limit', 0.22)
        self.declare_parameter('wall_follow_deadband', 0.07)
        self.declare_parameter('wall_follow_gain', 0.8)
        self.declare_parameter('use_kalman_filter', True)
        self.declare_parameter('kalman_process_noise', 0.01)
        self.declare_parameter('kalman_measurement_noise', 0.05)

        self.linear_speed = self.get_parameter('linear_speed').value
        self.turn_speed = self.get_parameter('turn_speed').value
        default_turn_direction = self.get_parameter('default_turn_direction').value
        self.default_turn_direction = -1.0 if default_turn_direction < 0.0 else 1.0
        self.front_stop_distance = self.get_parameter('front_stop_distance').value
        self.slow_down_distance = self.get_parameter('slow_down_distance').value
        self.turn_start_distance = self.get_parameter('turn_start_distance').value
        self.side_target_distance = self.get_parameter('side_target_distance').value
        self.min_points_required = self.get_parameter('min_points_required').value
        self.max_usable_distance = self.get_parameter('max_usable_distance').value
        self.front_axis = self.get_parameter('front_axis').value
        self.side_axis = self.get_parameter('side_axis').value
        self.vertical_axis = self.get_parameter('vertical_axis').value
        self.front_axis_sign = self.get_parameter('front_axis_sign').value
        self.side_axis_sign = self.get_parameter('side_axis_sign').value
        self.vertical_axis_sign = self.get_parameter('vertical_axis_sign').value
        self.front_half_width = self.get_parameter('front_half_width').value
        self.side_wall_min_abs_y = self.get_parameter('side_wall_min_abs_y').value
        self.side_min_forward = self.get_parameter('side_min_forward').value
        self.side_max_forward = self.get_parameter('side_max_forward').value
        self.vertical_min = self.get_parameter('vertical_min').value
        self.vertical_max = self.get_parameter('vertical_max').value
        self.right_memory_duration = self.get_parameter('right_memory_duration').value
        self.right_search_turn_speed = self.get_parameter(
            'right_search_turn_speed'
        ).value
        self.wall_follow_turn_limit = self.get_parameter(
            'wall_follow_turn_limit'
        ).value
        self.wall_follow_deadband = self.get_parameter('wall_follow_deadband').value
        self.wall_follow_gain = self.get_parameter('wall_follow_gain').value
        self.use_kalman = self.get_parameter('use_kalman_filter').value
        self.kalman_process_noise = self.get_parameter(
            'kalman_process_noise'
        ).value
        self.kalman_measurement_noise = self.get_parameter(
            'kalman_measurement_noise'
        ).value
        self.last_right_distance = math.inf
        self.last_right_seen_time = None
        self.kalman_front = KalmanFilter1D(
            process_noise=self.kalman_process_noise,
            measurement_noise=self.kalman_measurement_noise,
        )
        self.kalman_left = KalmanFilter1D(
            process_noise=self.kalman_process_noise,
            measurement_noise=self.kalman_measurement_noise,
        )
        self.kalman_right = KalmanFilter1D(
            process_noise=self.kalman_process_noise,
            measurement_noise=self.kalman_measurement_noise,
        )

        sensor_qos = QoSProfile(
            reliability=ReliabilityPolicy.BEST_EFFORT,
            history=HistoryPolicy.KEEP_LAST,
            depth=1,
        )
        cmd_qos = QoSProfile(
            reliability=ReliabilityPolicy.RELIABLE,
            history=HistoryPolicy.KEEP_LAST,
            depth=10,
        )

        self.cmd_pub = self.create_publisher(Twist, '/atlas/cmd_vel', cmd_qos)
        self.cloud_sub = self.create_subscription(
            PointCloud2,
            '/atlas/rgbd_camera/points',
            self.point_cloud_callback,
            sensor_qos,
        )

        self.get_logger().info(
            'Topic 2 depth wall follower started on '
            '/atlas/rgbd_camera/points -> /atlas/cmd_vel'
        )
        self.get_logger().info(
            f'Using front_axis={self.front_axis} sign={self.front_axis_sign}, '
            f'side_axis={self.side_axis} sign={self.side_axis_sign}, '
            f'vertical_axis={self.vertical_axis} sign={self.vertical_axis_sign}'
        )
        self.get_logger().info(
            f'Default turn direction: {self.default_turn_direction}'
        )
        self.get_logger().info('Turn convention: positive angular.z = LEFT')
        self.get_logger().info(
            'Right wall detection: '
            f'{self.side_min_forward:.2f}m < forward < '
            f'{self.side_max_forward:.2f}m, '
            f'abs(side) > {self.side_wall_min_abs_y:.2f}m, '
            f'memory={self.right_memory_duration:.1f}s'
        )
        self.get_logger().info(
            f'Kalman filter enabled={self.use_kalman} '
            f'q={self.kalman_process_noise:.3f} '
            f'r={self.kalman_measurement_noise:.3f}'
        )

    def point_cloud_callback(self, msg):
        try:
            points = self.read_xyz_points(msg)

            if points.shape[0] == 0:
                self.handle_sparse_cloud(0)
                return

            finite_mask = np.all(np.isfinite(points), axis=1)
            points = points[finite_mask]

            if points.shape[0] == 0:
                self.handle_sparse_cloud(0)
                return

            if points.shape[0] < self.min_points_required:
                self.handle_sparse_cloud(points.shape[0])
                return

            forward, side, vertical = self.extract_camera_axes(points)
            distance_mask = (
                (forward > 0.0) &
                (forward < self.max_usable_distance) &
                (np.abs(side) < self.max_usable_distance) &
                (vertical > self.vertical_min) &
                (vertical < self.vertical_max)
            )

            forward = forward[distance_mask]
            side = side[distance_mask]

            if forward.size < self.min_points_required:
                self.handle_sparse_cloud(forward.size)
                return

            # Camera-frame conventions vary. The official atlas/realsense cloud
            # uses +x as forward, y as sideways, and z as vertical. Adjust the
            # axis/sign parameters if a simulator build exposes a different frame.
            front_region = forward[np.abs(side) <= self.front_half_width]
            left_region = side[
                (side > self.side_wall_min_abs_y) &
                (forward > self.side_min_forward) &
                (forward < self.side_max_forward)
            ]
            right_region = -side[
                (side < -self.side_wall_min_abs_y) &
                (forward > self.side_min_forward) &
                (forward < self.side_max_forward)
            ]

            raw_front_distance = self.nearest_distance(front_region)
            raw_left_distance = self.nearest_distance(left_region)
            raw_right_distance = self.nearest_distance(right_region)
            front_distance = self.filter_front_distance(raw_front_distance)
            left_distance = self.filter_distance(self.kalman_left, raw_left_distance)
            filtered_right_distance = self.filter_distance(
                self.kalman_right,
                raw_right_distance,
            )
            right_distance, using_cached_right = self.effective_right_distance(
                filtered_right_distance
            )

            mode, turn_label = self.publish_control(
                front_distance,
                left_distance,
                right_distance,
            )
            self.log_distances(
                mode,
                turn_label,
                raw_front_distance,
                front_distance,
                left_distance,
                raw_right_distance,
                filtered_right_distance,
                right_distance,
                using_cached_right,
            )

        except Exception as exc:
            self.get_logger().error(f'Point cloud callback failed: {exc}')
            self.publish_stop()

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

    def extract_camera_axes(self, points):
        axis_indices = {'x': 0, 'y': 1, 'z': 2}
        front_index = axis_indices.get(self.front_axis, 0)
        side_index = axis_indices.get(self.side_axis, 1)
        vertical_index = axis_indices.get(self.vertical_axis, 2)

        forward = points[:, front_index] * self.front_axis_sign
        side = points[:, side_index] * self.side_axis_sign
        vertical = points[:, vertical_index] * self.vertical_axis_sign
        return forward, side, vertical

    def nearest_distance(self, values):
        if values.size == 0:
            return math.inf
        return float(np.min(values))

    def filter_distance(self, kalman_filter, measurement):
        if not self.use_kalman or not math.isfinite(measurement):
            return measurement
        return float(kalman_filter.filter_measurement(measurement))

    def filter_front_distance(self, measurement):
        filtered = self.filter_distance(self.kalman_front, measurement)

        # Keep front safety responsive: a sudden raw close obstacle should not
        # be hidden by Kalman lag.
        if math.isfinite(measurement) and math.isfinite(filtered):
            return min(measurement, filtered)
        return filtered

    def effective_right_distance(self, raw_right_distance):
        now = self.get_clock().now().nanoseconds / 1e9

        if math.isfinite(raw_right_distance):
            self.last_right_distance = raw_right_distance
            self.last_right_seen_time = now
            return raw_right_distance, False

        if self.last_right_seen_time is None:
            return math.inf, False

        age = now - self.last_right_seen_time
        if age <= self.right_memory_duration:
            return self.last_right_distance, True

        return math.inf, False

    def publish_control(self, front_distance, left_distance, right_distance):
        twist = Twist()
        turn_label = 'NONE'

        if front_distance < self.front_stop_distance:
            mode = 'HARD_STOP_TURN'
            twist.linear.x = 0.0
            twist.angular.z = float(self.turn_speed)
            turn_label = 'LEFT'
        elif front_distance < self.turn_start_distance:
            mode = 'FRONT_BLOCKED_TURN_LEFT'
            turn_progress = (
                (self.turn_start_distance - front_distance) /
                max(self.turn_start_distance - self.front_stop_distance, 0.01)
            )
            turn_progress = float(np.clip(turn_progress, 0.0, 1.0))
            twist.linear.x = float(np.interp(turn_progress, [0.0, 1.0], [0.06, 0.03]))
            twist.angular.z = float(
                np.interp(turn_progress, [0.0, 1.0], [0.25, self.turn_speed])
            )
            turn_label = 'LEFT'
        else:
            mode, angular_z, turn_label = self.right_wall_follow_command(
                right_distance
            )
            twist.angular.z = angular_z

            if front_distance < self.slow_down_distance:
                slow_progress = (
                    (self.slow_down_distance - front_distance) /
                    max(self.slow_down_distance - self.turn_start_distance, 0.01)
                )
                slow_progress = float(np.clip(slow_progress, 0.0, 1.0))
                twist.linear.x = float(
                    np.interp(
                        slow_progress,
                        [0.0, 1.0],
                        [self.linear_speed, 0.06],
                    )
                )
            else:
                twist.linear.x = float(self.linear_speed)

            if mode == 'RIGHT_TOO_FAR_SEARCH':
                twist.linear.x = min(twist.linear.x, 0.08)

        twist.linear.x = max(0.0, float(twist.linear.x))
        twist.angular.z = float(
            np.clip(twist.angular.z, -self.turn_speed, self.turn_speed)
        )
        self.cmd_pub.publish(twist)
        return mode, turn_label

    def right_wall_follow_command(self, right_distance):
        if not math.isfinite(right_distance):
            angular_z = -min(self.right_search_turn_speed, self.turn_speed)
            return 'RIGHT_TOO_FAR_SEARCH', angular_z, 'RIGHT'

        distance_error = right_distance - self.side_target_distance
        turn_limit = min(self.wall_follow_turn_limit, self.turn_speed)

        if abs(distance_error) <= self.wall_follow_deadband:
            angular_z = float(
                np.clip(-0.4 * self.wall_follow_gain * distance_error,
                        -turn_limit,
                        turn_limit)
            )
            return 'WALL_FOLLOW_CLEAR', angular_z, self.turn_label_from_angular(
                angular_z
            )

        angular_z = float(
            np.clip(-self.wall_follow_gain * distance_error,
                    -turn_limit,
                    turn_limit)
        )
        if distance_error < 0.0:
            mode = 'RIGHT_TOO_CLOSE'
        else:
            mode = 'RIGHT_TOO_FAR'
        return mode, angular_z, self.turn_label_from_angular(angular_z)

    def side_error(self, left_distance, right_distance):
        if not math.isfinite(left_distance) and not math.isfinite(right_distance):
            return 0.0
        return (
            self.safe_side_distance(left_distance) -
            self.safe_side_distance(right_distance)
        )

    def safe_side_distance(self, distance):
        if math.isfinite(distance):
            return distance
        return self.side_target_distance

    def choose_escape_turn(self, left_distance, right_distance, turn_speed):
        turn_label = self.choose_escape_turn_label(left_distance, right_distance)
        angular_z = self.angular_for_turn_label(turn_label, turn_speed)
        if turn_label == 'DEFAULT':
            turn_label = self.default_turn_label()
        return angular_z, turn_label

    def choose_escape_turn_label(self, left_distance, right_distance):
        left_known = math.isfinite(left_distance)
        right_known = math.isfinite(right_distance)

        if left_known and right_known:
            if self.is_ambiguous_turn(left_distance, right_distance):
                return 'DEFAULT'
            if left_distance > right_distance:
                return 'LEFT'
            return 'RIGHT'

        if right_known and right_distance < self.side_target_distance:
            return 'LEFT'
        if left_known and left_distance < self.side_target_distance:
            return 'RIGHT'

        return 'DEFAULT'

    def angular_for_turn_label(self, turn_label, turn_speed):
        # ROS/Gazebo uses positive angular.z for a left turn and negative for
        # a right turn.
        if turn_label == 'LEFT':
            return float(turn_speed)
        if turn_label == 'RIGHT':
            return -float(turn_speed)
        return float(self.default_turn_direction * turn_speed)

    def default_turn_label(self):
        return 'LEFT' if self.default_turn_direction > 0.0 else 'RIGHT'

    def turn_label_from_angular(self, angular_z):
        if abs(angular_z) < 0.01:
            return 'NONE'
        if angular_z > 0.0:
            return 'LEFT'
        return 'RIGHT'

    def is_ambiguous_turn(self, left_distance, right_distance):
        if not math.isfinite(left_distance) and not math.isfinite(right_distance):
            return True
        if math.isfinite(left_distance) != math.isfinite(right_distance):
            return False
        return abs(left_distance - right_distance) < 0.01

    def handle_sparse_cloud(self, point_count):
        self.get_logger().warning(
            'LOW_POINTS_ROTATE: '
            f'point cloud has too few valid points ({point_count}); rotating slowly',
            throttle_duration_sec=2.0,
        )
        twist = Twist()
        twist.linear.x = 0.0
        twist.angular.z = float(min(self.turn_speed, 0.20))
        self.cmd_pub.publish(twist)

    def publish_stop(self):
        self.cmd_pub.publish(Twist())

    def log_distances(
        self,
        mode,
        turn_label,
        raw_front_distance,
        front_distance,
        left_distance,
        raw_right_distance,
        filtered_right_distance,
        right_distance,
        using_cached_right,
    ):
        self.get_logger().info(
            f'mode={mode} '
            f'raw_front={self.format_distance(raw_front_distance)}m '
            f'front={self.format_distance(front_distance)}m '
            f'left={self.format_distance(left_distance)}m '
            f'raw_right={self.format_distance(raw_right_distance)}m '
            f'filtered_right={self.format_distance(filtered_right_distance)}m '
            f'effective_right={self.format_distance(right_distance)}m '
            f'cached_right={using_cached_right} '
            f'turn={turn_label}',
            throttle_duration_sec=2.0,
        )

    def format_distance(self, distance):
        if math.isfinite(distance):
            return f'{distance:.2f}'
        return 'inf'


def main(args=None):
    rclpy.init(args=args)
    node = Topic2DepthWallFollower()

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
