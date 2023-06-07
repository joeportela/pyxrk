from typing import List
import os

from pyxrk.xrk import aim_xrk, ctypes



class Session:
    def __init__(self, file_ptr) -> None:
        self._xrk = aim_xrk.open_file(file_ptr.value)
        
    def get_channel_names(self) -> List[str]:
        channel_names = []
        
        for i in range(self.get_channel_count()):
            import pdb;pdb.set_trace()
            ptr = aim_xrk.get_channel_name(self._xrk, i)
            channel_i = ctypes.c_char_p(ptr)
            channel_names.append(channel_i)
        
        return channel_names

    def get_channel_count(self) -> int:
        return aim_xrk.get_channels_count(self._xrk)



def load_session(file_path: str) -> Session:
    abs_path = os.path.abspath(os.path.expanduser(file_path))
    file_ptr = ctypes.c_char_p(abs_path.encode())
    return Session(file_ptr)

