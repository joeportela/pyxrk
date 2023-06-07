import platform

if platform.system() == 'Windows':
    import ctypes
else:
    from zugbruecke import CtypesSession  # type: ignore
    ctypes = CtypesSession(arch = 'win64')

aim_xrk = ctypes.cdll.LoadLibrary('aim/MatLabXRK-2017-64-ReleaseU.dll')
