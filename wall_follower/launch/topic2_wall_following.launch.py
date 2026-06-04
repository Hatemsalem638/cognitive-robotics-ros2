import os

from ament_index_python.packages import get_package_share_directory
from launch import LaunchDescription
from launch.actions import DeclareLaunchArgument
from launch.substitutions import LaunchConfiguration
from launch_ros.actions import Node


def generate_launch_description():
    """Launch the existing wall follower on the official Route A atlas robot."""

    wall_follower_params = DeclareLaunchArgument(
        'wall_follower_params',
        default_value=os.path.join(
            get_package_share_directory('wall_follower'),
            'config',
            'wall_follower.yaml',
        ),
        description='Path to wall-follower config file',
    )

    wall_follower_node = Node(
        package='wall_follower',
        executable='wall_follower_node',
        name='topic2_route_a_wall_follower',
        output='screen',
        parameters=[LaunchConfiguration('wall_follower_params')],
        remappings=[
            ('/cmd_vel', '/atlas/cmd_vel'),
        ],
    )

    return LaunchDescription([
        wall_follower_params,
        wall_follower_node,
    ])
