import datetime

import pytest
import pyxrk

TEST_FILE = "./test/data/test.xrk"


def test_load_run():
    pyxrk.load_run(TEST_FILE)
    with pytest.raises(ValueError, match="does not exist"):
        pyxrk.load_run("./non-existant-file.xrk")


def test_run_attributes():
    run = pyxrk.load_run(TEST_FILE)
    assert run.lap_count == 7
    assert run.championship == ""
    assert run.track == "Watkins Glen"
    assert run.venue_type == ""
    assert run.vehicle == "Porsche 981 GT4"
    assert run.racer == "Joe Portela"
    assert run.datetime == datetime.datetime(2023, 5, 11, 16, 5, 9)
