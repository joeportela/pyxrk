# pyxrk
Python package for reading .xrk files. It wraps much of the functionality from the rust library [xdrk](https://github.com/bmc-labs/xdrk) while adding some convenience functionality.

---

Install pyxrk using pip:

```shell
$ pip install pyxrk
```

To get started with basic usage:

```pycon
>>> import pyxrk
>>> run = pyxrk.load_run("./my_run.xrk")
>>> run.lap_count
7
>>> run.racer
'Lewis Hamilton'
>>> channel = run.get_channel("GPS Speed")
>>> channel.unit
'm/s'
>>> channel.frequency  # in Hz
100.0
>>> for data in channel.data():
...     print(data)  # tuple of (time offset, data point)
...     break
...
(0.0, 0.1846618503332138)
```

In lieau of better API documentation, see `pyxrk.pyi` for full interface and `test/test_run.py` for more example usage.


## Compatibility

Currently known to work on Linux and Windows, but not Mac.  Contributions for getting it working on Mac are more than welcome.