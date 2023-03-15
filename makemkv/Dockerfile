FROM localhost/ffmpeg-qs
WORKDIR /makemkv
ENV DEBIAN_FRONTEND="noninteractive"
RUN apt-get update -qq \
    && apt-get install -y \
        build-essential \
        pkg-config \
        libc6-dev \
        libssl-dev \
        libexpat1-dev \
        libgl1-mesa-dev \
        qtbase5-dev \
        zlib1g-dev
RUN wget https://www.makemkv.com/download/makemkv-bin-1.17.3.tar.gz \
    && wget https://www.makemkv.com/download/makemkv-oss-1.17.3.tar.gz \
    && tar xzf *.tar.gz \
    && cd makemkv-oss-1.17.3 \
    && PKG_CONFIG_PATH=/root/ffmpeg_build/lib/pkgconfig ./configure \
    && make && make install \
    && cd .. \
    && cd makemkv-bin-1.17.3 \
    && make && make install
ENTRYPOINT [ "/usr/bin/makemkvcon" ]
CMD [ "-h" ]
