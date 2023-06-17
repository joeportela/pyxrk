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


def test_channel_names():
    run = pyxrk.load_run(TEST_FILE)
    assert len(run.channel_names) == 57
    assert run.channels_count == 57
    assert run.get_channel_idx("TRQ_LOSS") == 13
    assert run.get_channel_idx("OIL_TEMP") == 42
    assert run.get_channel_idx("GPS Speed") == 46
    assert run.get_channel_idx("GPS Radius") == 56
    with pytest.raises(ValueError, match="Channel not found"):
        run.get_channel_idx("foo")


def test_channel_unit():
    run = pyxrk.load_run(TEST_FILE)
    assert run.get_channel_unit("OIL_TEMP") == "C"
    assert run.get_channel_unit("GPS Speed") == "m/s"
    with pytest.raises(ValueError, match="Channel not found"):
        run.get_channel_unit("foo")
    assert run.get_channel_unit_by_idx(42) == "C"
    assert run.get_channel_unit_by_idx(46) == "m/s"
    with pytest.raises(ValueError, match="Couldn't get channel unit"):
        run.get_channel_unit_by_idx(1000)