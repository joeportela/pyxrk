import datetime

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

    def get_channel_idx(channel_name: str) -> int: ...
    def get_channel_unit(channel_name: str) -> str: ...
    def get_channel_unit_by_idx(idx: int) -> str: ...

class Channel: ...

def load_run(path: str) -> Run: ...
