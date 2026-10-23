from setuptools import find_packages
from setuptools import setup

setup(
    name='yolo_msgs',
    version='0.1.0',
    packages=find_packages(
        include=('yolo_msgs', 'yolo_msgs.*')),
)
