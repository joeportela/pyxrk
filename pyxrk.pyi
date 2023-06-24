import datetime
from collections.abc import Iterable, Iterator
from typing import Tuple

from pyarrow import Array

class ChannelDataIterator(Iterable):
    def __iter__(self) -> Iterator[Tuple[float, float]]: ...

class Channel:
    name: str
    unit: str
    frequency: float

    def sample_count(self) -> int: ...
    def is_empty(self) -> bool: ...
    def sync_with(self, other: "Channel") -> "Channel": ...
    def samples(self) -> list[float]: ...
    def timestamps(self) -> list[float]: ...
    def data(self) -> ChannelDataIterator: ...
    def to_array(self) -> Array: ...

class Run:
    lap_count: int
    championship: str
    track: str
    venue_type: str
    vehicle: str
    racer: str
    datetime: datetime.datetime
    channel_names: list[str]
    channels_count: int

    def get_channel_idx(self, channel_name: str) -> int: ...
    def get_channel_unit(self, channel_name: str) -> str: ...
    def get_channel_unit_by_idx(self, idx: int) -> str: ...
    def get_channel(self, channel_name: str, lap: int | None = None) -> Channel: ...
    def get_channel_by_idx(self, idx: int, lap: int | None = None) -> Channel: ...

def load_run(path: str) -> Run: ...
