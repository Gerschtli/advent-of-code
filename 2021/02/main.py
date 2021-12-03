from __future__ import annotations

from dataclasses import dataclass
from enum import Enum
from typing import Tuple


class Direction(Enum):
    DOWN = "down"
    FORWARD = "forward"
    UP = "up"


@dataclass(frozen=True)
class Command:
    direction: Direction
    size: int


@dataclass(frozen=True)
class Position:
    x: int = 0
    y: int = 0

    def move(self, command: Command) -> Position:
        x = self.x
        y = self.y

        if command.direction == Direction.FORWARD:
            x += command.size
        elif command.direction == Direction.UP:
            y -= command.size
        elif command.direction == Direction.DOWN:
            y += command.size

        return Position(x, y)


@dataclass(frozen=True)
class PositionWithAim:
    aim: int = 0
    x: int = 0
    y: int = 0

    def move(self, command: Command) -> PositionWithAim:
        aim = self.aim
        x = self.x
        y = self.y

        if command.direction == Direction.DOWN:
            aim += command.size
        elif command.direction == Direction.UP:
            aim -= command.size
        elif command.direction == Direction.FORWARD:
            x += command.size
            y += command.size * aim

        return PositionWithAim(aim, x, y)


def read_file(path: str) -> [str]:
    with open(path, 'r') as file:
        return [line.strip() for line in file.readlines()]


def parse_commands(lines: [str]) -> [Command]:
    commands = []
    for line in lines:
        direction, size = line.split(" ")

        commands.append(
            Command(
                direction=Direction[direction.upper()],
                size=int(size)
            )
        )

    return commands


def evaluate_positions(path: str) -> Tuple[Position, PositionWithAim]:
    lines = read_file(path)
    commands = parse_commands(lines)

    position = Position()
    position_with_aim = PositionWithAim()
    for command in commands:
        position = position.move(command)
        position_with_aim = position_with_aim.move(command)

    return position, position_with_aim


def main() -> None:
    position, position_with_aim = evaluate_positions('./files/input.txt')

    print(f'multiplication of x and y: {position.x * position.y}')
    print(f'multiplication of x and y with aim: {position_with_aim.x * position_with_aim.y}')


if __name__ == '__main__':
    main()
