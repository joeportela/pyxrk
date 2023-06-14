import datetime

class Run:
    lap_count: int
    championship: str
    track: str
    venue_type: str
    vehicle: str
    racer: str
    datetime: datetime.datetime

def load_run(path: str) -> Run: ...
