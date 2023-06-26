from pyxrk import Run

TEST_FILE = "./test/data/test.xrk"


def test_load_run() -> None:
    run = Run.load(TEST_FILE)
    run.get_lap(1)
