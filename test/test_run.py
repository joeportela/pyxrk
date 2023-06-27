from pyxrk import Run

TEST_FILE = "./test/data/test.xrk"


def test_lap_table() -> None:
    run = Run.load(TEST_FILE)
    lap = run.get_lap(3)
    lap.to_table()


def test_run_table() -> None:
    run = Run.load(TEST_FILE)
    run.to_table()
