import json

import rclpy
from geometry_msgs.msg import Twist
from nav2_msgs.msg import SpeedLimit
from rclpy.duration import Duration
from rclpy.executors import ExternalShutdownException
from rclpy.node import Node
from std_msgs.msg import String


class Topic2Nav2TrafficRules(Node):
    """Apply YOLO traffic-sign detections to Nav2 speed control."""

    def __init__(self):
        super().__init__('topic2_nav2_traffic_rules')

        self.declare_parameter('min_confidence', 0.45)
        self.declare_parameter('slow_speed', 0.05)
        self.declare_parameter('fast_speed', 0.28)
        self.declare_parameter('normal_speed', 0.18)
        self.declare_parameter('stop_duration', 3.0)
        self.declare_parameter('rule_hold_duration', 2.5)

        self.min_confidence = float(self.get_parameter('min_confidence').value)
        self.slow_speed = float(self.get_parameter('slow_speed').value)
        self.fast_speed = float(self.get_parameter('fast_speed').value)
        self.normal_speed = float(self.get_parameter('normal_speed').value)
        self.stop_duration = float(self.get_parameter('stop_duration').value)
        self.rule_hold_duration = float(
            self.get_parameter('rule_hold_duration').value
        )

        self.speed_pub = self.create_publisher(SpeedLimit, '/speed_limit', 10)
        self.state_pub = self.create_publisher(String, '/traffic_rule_state', 10)
        self.zero_pub = self.create_publisher(Twist, '/atlas/cmd_vel', 10)

        self.create_subscription(
            String,
            '/yolo/detections_json',
            self.detections_cb,
            10,
        )

        self.stop_until = None
        self.rule_until = None
        self.current_rule = 'NORMAL'

        self.create_timer(0.1, self.timer_cb)

        self.publish_speed_limit(self.normal_speed)
        self.publish_state('NORMAL', self.normal_speed)
        self.get_logger().info(
            'Topic 2 Nav2 traffic rules started: '
            '/yolo/detections_json -> /speed_limit'
        )

    def detections_cb(self, msg):
        try:
            detections = json.loads(msg.data)
        except Exception as exc:
            self.get_logger().warn(f'Ignoring invalid YOLO JSON: {exc}')
            return

        detections = [
            detection
            for detection in detections
            if float(detection.get('conf', 0.0)) >= self.min_confidence
        ]
        if not detections:
            return

        best = max(detections, key=lambda item: float(item.get('conf', 0.0)))
        label = str(best.get('label', '')).strip().lower()
        confidence = float(best.get('conf', 0.0))
        rule = self.label_to_rule(label)

        if rule is None:
            return

        now = self.get_clock().now()

        if rule == 'STOP':
            self.stop_until = now + Duration(seconds=self.stop_duration)
            self.rule_until = self.stop_until
            self.current_rule = 'STOP'
            self.publish_speed_limit(0.01)
            self.publish_state('STOP', 0.0)
            self.zero_pub.publish(Twist())
            self.get_logger().warn(
                f'STOP detected ({label}, conf={confidence:.2f}): '
                f'holding for {self.stop_duration:.1f}s'
            )
            return

        if rule == 'SLOW':
            speed = self.slow_speed
        elif rule == 'FAST':
            speed = self.fast_speed
        else:
            speed = self.normal_speed

        self.current_rule = rule
        self.rule_until = now + Duration(seconds=self.rule_hold_duration)
        self.publish_speed_limit(speed)
        self.publish_state(rule, speed)
        self.get_logger().info(
            f'{rule} detected ({label}, conf={confidence:.2f}): '
            f'speed_limit={speed:.2f}'
        )

    @staticmethod
    def label_to_rule(label):
        mapping = {
            'stop': 'STOP',
            'stopsign': 'STOP',
            'stop_sign': 'STOP',
            'slow': 'SLOW',
            'slowsign': 'SLOW',
            'slow_sign': 'SLOW',
            'fast': 'FAST',
            'fastsign': 'FAST',
            'fast_sign': 'FAST',
        }
        return mapping.get(label)

    def timer_cb(self):
        now = self.get_clock().now()

        if self.stop_until is not None:
            if now < self.stop_until:
                self.zero_pub.publish(Twist())
                return

            self.stop_until = None
            self.restore_normal('STOP finished')
            return

        if self.rule_until is not None and now >= self.rule_until:
            if self.current_rule != 'NORMAL':
                self.restore_normal('Traffic rule expired')

    def restore_normal(self, reason):
        self.current_rule = 'NORMAL'
        self.rule_until = None
        self.publish_speed_limit(self.normal_speed)
        self.publish_state('NORMAL', self.normal_speed)
        self.get_logger().info(
            f'{reason}: restored speed_limit={self.normal_speed:.2f}'
        )

    def publish_speed_limit(self, speed):
        msg = SpeedLimit()
        msg.percentage = False
        msg.speed_limit = float(speed)
        self.speed_pub.publish(msg)

    def publish_state(self, rule, speed):
        msg = String()
        msg.data = json.dumps(
            {
                'rule': rule,
                'speed_limit': float(speed),
            }
        )
        self.state_pub.publish(msg)


def main(args=None):
    rclpy.init(args=args)
    node = Topic2Nav2TrafficRules()
    try:
        rclpy.spin(node)
    except (KeyboardInterrupt, ExternalShutdownException):
        pass
    node.destroy_node()
    if rclpy.ok():
        rclpy.shutdown()


if __name__ == '__main__':
    main()
