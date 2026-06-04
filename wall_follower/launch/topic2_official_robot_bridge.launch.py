from launch import LaunchDescription
from launch_ros.actions import Node


def generate_launch_description():
    """Launch the ROS-GZ bridge for the official Route A simulated robot."""

    bridge_node = Node(
        package='ros_gz_bridge',
        executable='parameter_bridge',
        name='topic2_route_a_bridge',
        output='screen',
        arguments=[
            '/clock@rosgraph_msgs/msg/Clock[ignition.msgs.Clock',
            '/atlas/odom_ground_truth@nav_msgs/msg/Odometry[ignition.msgs.Odometry',
            '/atlas/rgbd_camera/image@sensor_msgs/msg/Image[ignition.msgs.Image',
            '/atlas/rgbd_camera/depth_image@sensor_msgs/msg/Image[ignition.msgs.Image',
            '/atlas/rgbd_camera/camera_info@sensor_msgs/msg/CameraInfo[ignition.msgs.CameraInfo',
            '/atlas/rgbd_camera/points@sensor_msgs/msg/PointCloud2[ignition.msgs.PointCloudPacked',
            '/atlas/imu/data@sensor_msgs/msg/Imu[ignition.msgs.IMU',
            '/model/atlas/odometry@nav_msgs/msg/Odometry[ignition.msgs.Odometry',
            '/model/atlas/tf@tf2_msgs/msg/TFMessage[ignition.msgs.Pose_V',
            '/atlas/cmd_vel@geometry_msgs/msg/Twist]ignition.msgs.Twist',
        ],
    )

    return LaunchDescription([
        bridge_node,
    ])
