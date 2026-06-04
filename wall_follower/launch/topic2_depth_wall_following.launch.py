from launch import LaunchDescription
from launch_ros.actions import Node


def generate_launch_description():
    """Launch Route A depth-based wall following for the official atlas robot."""

    depth_wall_follower = Node(
        package='wall_follower',
        executable='topic2_depth_wall_follower',
        name='topic2_depth_wall_follower',
        output='screen',
    )

    return LaunchDescription([
        depth_wall_follower,
    ])
