from launch import LaunchDescription
from launch_ros.actions import Node


def generate_launch_description():
    """Launch Route A enhanced navigation for the official atlas robot."""

    enhanced_navigation = Node(
        package='wall_follower',
        executable='topic2_enhanced_navigation',
        name='topic2_enhanced_navigation',
        output='screen',
    )

    return LaunchDescription([
        enhanced_navigation,
    ])
