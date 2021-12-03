from main import Command, Direction, Position, evaluate_position, parse_commands, read_file


def test_read_file_returns_lines_of_file_as_list() -> None:
    lines = read_file('./files/input-example.txt')

    assert lines == [
        'forward 5',
        'down 5',
        'forward 8',
        'up 3',
        'down 8',
        'forward 2',
    ]


def test_parse_commands_returns_list_of_commands_with_all_directions() -> None:
    commands = parse_commands(
        [
            'forward 17',
            'down 5',
            'up 3',
        ]
    )

    assert commands == [
        Command(Direction.FORWARD, 17),
        Command(Direction.DOWN, 5),
        Command(Direction.UP, 3),
    ]


def test_position_inits_with_x_0_and_y_0() -> None:
    position = Position()

    assert position.x == 0
    assert position.y == 0


def test_position_move_increases_x_value_given_a_forward_command() -> None:
    position = Position()

    new_position = position.move(Command(Direction.FORWARD, 5))

    assert new_position.x == 5
    assert new_position.y == 0


def test_position_move_increases_y_value_given_a_down_command() -> None:
    position = Position()

    new_position = position.move(Command(Direction.DOWN, 2))

    assert new_position.x == 0
    assert new_position.y == 2


def test_position_move_decreases_y_value_given_a_up_command() -> None:
    position = Position()

    new_position = position.move(Command(Direction.UP, 2))

    assert new_position.x == 0
    assert new_position.y == -2


def test_evaluate_position_returns_x_15_y_10_for_example() -> None:
    position = evaluate_position('./files/input-example.txt')

    assert position.x == 15
    assert position.y == 10


def test_evaluate_position_returns_x_1968_y_1063_for_input() -> None:
    position = evaluate_position('./files/input.txt')

    assert position.x == 1968
    assert position.y == 1063
