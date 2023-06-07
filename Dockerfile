FROM ubuntu:20.04

RUN apt-get update -y && \
    apt-get install -y --no-install-recommends \
        software-properties-common \
        ca-certificates \
        wget

RUN dpkg --add-architecture i386 && \
    mkdir -pm755 /etc/apt/keyrings && \
    wget -O /etc/apt/keyrings/winehq-archive.key https://dl.winehq.org/wine-builds/winehq.key && \
    wget -nc -P /etc/apt/sources.list.d/ https://dl.winehq.org/wine-builds/ubuntu/dists/$(lsb_release -sc)/winehq-$(lsb_release -sc).sources && \
    apt-get update -y && \
    apt-get install -y --install-recommends winehq-stable

# Install Python 3.11
RUN add-apt-repository -y ppa:deadsnakes/ppa && \
    apt-get install -y python3.11 && \
    update-alternatives --install /usr/bin/python3 python3 /usr/bin/python3.11 1 && \
    update-alternatives --config python3 && \
    wget -O - https://bootstrap.pypa.io/get-pip.py | python3

# Install zugbruecke and preload a session (first time takes a while)
RUN pip install zugbruecke && \
    python3 -c "from zugbruecke import CtypesSession; CtypesSession(arch = 'win64')"

COPY . /app

ENTRYPOINT ["tail", "-f", "/dev/null"]
