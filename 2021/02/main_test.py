from main import Command, Direction, Position, PositionWithAim, evaluate_positions, parse_commands, read_file


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


def test_position_move_decreases_y_value_given_an_up_command() -> None:
    position = Position()

    new_position = position.move(Command(Direction.UP, 2))

    assert new_position.x == 0
    assert new_position.y == -2


def test_position_with_aim_inits_with_aim_x_and_y_0() -> None:
    position = PositionWithAim()

    assert position.aim == 0
    assert position.x == 0
    assert position.y == 0


def test_position_with_aim_increases_aim_given_a_down_command() -> None:
    position = PositionWithAim()

    new_position = position.move(Command(Direction.DOWN, 3))

    assert new_position.aim == 3
    assert new_position.x == 0
    assert new_position.y == 0


def test_position_with_aim_decreases_aim_given_an_up_command() -> None:
    position = PositionWithAim()

    new_position = position.move(Command(Direction.UP, 2))

    assert new_position.aim == -2
    assert new_position.x == 0
    assert new_position.y == 0


def test_position_with_aim_changes_x_and_y_given_an_forward_command() -> None:
    position = PositionWithAim(aim=5)

    new_position = position.move(Command(Direction.FORWARD, 3))

    assert new_position.aim == 5
    assert new_position.x == 3
    assert new_position.y == 15


def test_evaluate_positions_for_example() -> None:
    position, position_with_aim = evaluate_positions('./files/input-example.txt')

    assert position.x == 15
    assert position.y == 10

    assert position_with_aim.x == 15
    assert position_with_aim.y == 60


def test_evaluate_positions_for_input() -> None:
    position, position_with_aim = evaluate_positions('./files/input.txt')

    assert position.x == 1968
    assert position.y == 1063

    assert position_with_aim.x == 1968
    assert position_with_aim.y == 1060092
