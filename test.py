from pyxrk.session import load_session

session = load_session("~/Downloads/test.xrk")
print(session.get_channel_count())
print(session.get_channel_names())