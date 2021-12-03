from __future__ import annotations

from dataclasses import dataclass
from enum import Enum


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


def read_file(path: str) -> [str]:
    with open(path, 'r') as file:
        return [line.strip() for line in file.readlines()]


def parse_commands(lines: [str]) -> [Command]:
    commands = []
    for line in lines:
        [direction, size] = line.split(" ")

        commands.append(
            Command(
                direction=Direction[direction.upper()],
                size=int(size)
            )
        )

    return commands


def evaluate_position(path: str) -> Position:
    lines = read_file(path)
    commands = parse_commands(lines)

    position = Position()
    for command in commands:
        position = position.move(command)

    return position


def main() -> None:
    position = evaluate_position('./files/input.txt')

    print(f'multiplication of x and y: {position.x * position.y}')


if __name__ == '__main__':
    main()
