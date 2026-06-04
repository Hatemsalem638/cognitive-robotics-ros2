#!/usr/bin/env python3
"""Create a Route A Nav2 map with occupied cells from the official maze SDF."""

from __future__ import annotations

import math
import xml.etree.ElementTree as ET
from pathlib import Path


WORLD_SDF = Path(
    '/work/install/ntu_robotsim/share/ntu_robotsim/worlds/cwmaze/cwmazenew.sdf'
)
OUTPUT_DIR = Path('/work/wall_follower/maps')
MAP_NAME = 'topic2_nav2_clean_map'

RESOLUTION = 0.08
ORIGIN_X = -5.6
ORIGIN_Y = -5.6
MAX_X = 10.6
MAX_Y = 5.6

FREE = 254
OCCUPIED = 0
UNKNOWN = 205


def parse_vector(text: str) -> list[float]:
    return [float(value) for value in text.split()]


def official_wall_boxes(world_sdf: Path) -> list[tuple[str, float, float, float, float]]:
    tree = ET.parse(world_sdf)
    root = tree.getroot()
    maze = root.find(".//model[@name='maze']")
    if maze is None:
        raise RuntimeError(f'Could not find model name="maze" in {world_sdf}')

    walls = []
    for link in maze.findall('link'):
        name = link.attrib.get('name', '')
        if not name.startswith('wall_'):
            continue

        pose_text = link.findtext('pose')
        size_text = link.findtext('./collision/geometry/box/size')
        if pose_text is None or size_text is None:
            continue

        pose = parse_vector(pose_text)
        size = parse_vector(size_text)
        if len(pose) < 2 or len(size) < 2:
            continue

        walls.append((name, pose[0], pose[1], size[0], size[1]))

    if not walls:
        raise RuntimeError(f'No wall boxes found in {world_sdf}')
    return walls


def create_grid(width: int, height: int) -> bytearray:
    return bytearray([FREE] * (width * height))


def cell_center(col: int, row: int, height: int) -> tuple[float, float]:
    x = ORIGIN_X + (col + 0.5) * RESOLUTION
    y = ORIGIN_Y + (height - row - 0.5) * RESOLUTION
    return x, y


def draw_wall(
    grid: bytearray,
    width: int,
    height: int,
    cx: float,
    cy: float,
    sx: float,
    sy: float,
) -> None:
    x_min = cx - sx * 0.5
    x_max = cx + sx * 0.5
    y_min = cy - sy * 0.5
    y_max = cy + sy * 0.5

    for row in range(height):
        for col in range(width):
            x, y = cell_center(col, row, height)
            if x_min <= x <= x_max and y_min <= y <= y_max:
                grid[row * width + col] = OCCUPIED


def write_pgm(path: Path, width: int, height: int, grid: bytearray) -> None:
    with path.open('wb') as stream:
        stream.write(f'P5\n{width} {height}\n255\n'.encode('ascii'))
        stream.write(grid)


def write_yaml(path: Path) -> None:
    path.write_text(
        f"""image: {MAP_NAME}.pgm
mode: trinary
resolution: {RESOLUTION}
origin: [{ORIGIN_X}, {ORIGIN_Y}, 0]
negate: 0
occupied_thresh: 0.65
free_thresh: 0.25
""",
        encoding='ascii',
    )


def print_counts(grid: bytearray) -> None:
    occupied = sum(value == OCCUPIED for value in grid)
    free = sum(value == FREE for value in grid)
    unknown = sum(value == UNKNOWN for value in grid)
    print(f'occupied cells: {occupied}')
    print(f'free cells: {free}')
    print(f'unknown cells: {unknown}')
    print(f'total cells: {len(grid)}')


def main() -> None:
    width = math.ceil((MAX_X - ORIGIN_X) / RESOLUTION)
    height = math.ceil((MAX_Y - ORIGIN_Y) / RESOLUTION)
    grid = create_grid(width, height)

    walls = official_wall_boxes(WORLD_SDF)
    for _, cx, cy, sx, sy in walls:
        draw_wall(grid, width, height, cx, cy, sx, sy)

    OUTPUT_DIR.mkdir(parents=True, exist_ok=True)
    pgm_path = OUTPUT_DIR / f'{MAP_NAME}.pgm'
    yaml_path = OUTPUT_DIR / f'{MAP_NAME}.yaml'

    write_pgm(pgm_path, width, height, grid)
    write_yaml(yaml_path)

    print(f'created: {pgm_path}')
    print(f'created: {yaml_path}')
    print(f'resolution: {RESOLUTION}')
    print(f'width: {width}')
    print(f'height: {height}')
    print(f'origin: [{ORIGIN_X}, {ORIGIN_Y}, 0]')
    print_counts(grid)


if __name__ == '__main__':
    main()
