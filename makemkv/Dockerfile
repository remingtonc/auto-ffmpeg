FROM docker.io/intel/oneapi-basekit
ENV DEBIAN_FRONTEND="noninteractive"
RUN apt-get update -qq \
    && apt-get -y install \
        autoconf \
        automake \
        build-essential \
        cmake \
        git-core \
        libass-dev \
        libfreetype6-dev \
        libgnutls28-dev \
        libmp3lame-dev \
        libtool \
        libvorbis-dev \
        meson \
        ninja-build \
        pkg-config \
        texinfo \
        wget \
        yasm \
        zlib1g-dev \
        libunistring-dev \
        libvpl-dev \
    && mkdir -p ~/ffmpeg_sources ~/bin
# nasm is 2.15 in jammy so has avx-512 and unlikely to have qs specials, just install
RUN apt-get install -y nasm libfdk-aac-dev
# Compile libx264
RUN cd ~/ffmpeg_sources \
    && git -C x264 pull 2> /dev/null || git clone --depth 1 https://code.videolan.org/videolan/x264.git \
    && cd x264 \
    && PATH="$HOME/bin:$PATH" PKG_CONFIG_PATH="$HOME/ffmpeg_build/lib/pkgconfig" ./configure --prefix="$HOME/ffmpeg_build" --bindir="$HOME/bin" --enable-static --enable-pic \
    && PATH="$HOME/bin:$PATH" make -j$(($(nproc)-1)) \
    && make install
# Compile libx265
RUN apt-get install -y libnuma-dev \
    && cd ~/ffmpeg_sources \
    && wget -O x265.tar.bz2 https://bitbucket.org/multicoreware/x265_git/get/master.tar.bz2 \
    && tar xjvf x265.tar.bz2 \
    && cd multicoreware*/build/linux \
    && PATH="$HOME/bin:$PATH" cmake -G "Unix Makefiles" -DCMAKE_INSTALL_PREFIX="$HOME/ffmpeg_build" -DENABLE_SHARED=off ../../source \
    && PATH="$HOME/bin:$PATH" make -j$(($(nproc)-1)) \
    && make install
# Compile libvmaf because it seems nifty
RUN cd ~/ffmpeg_sources \
    && wget https://github.com/Netflix/vmaf/archive/v2.1.1.tar.gz \
    && tar xvf v2.1.1.tar.gz \
    && mkdir -p vmaf-2.1.1/libvmaf/build \
    && cd vmaf-2.1.1/libvmaf/build \
    && meson setup -Denable_tests=false -Denable_docs=false --buildtype=release --default-library=static .. --prefix "$HOME/ffmpeg_build" --bindir="$HOME/ffmpeg_build/bin" --libdir="$HOME/ffmpeg_build/lib" \
    && ninja -j$(($(nproc)-1)) \
    && ninja install
# Uhh let's try installing the GPU kit
RUN apt-get update \
    && apt-get -y install \
        cmake \
        libdrm-dev libdrm-intel1 \
        libva-dev libva2 libva-drm2 \
        libvpl-dev \
    && git clone https://github.com/oneapi-src/oneVPL-intel-gpu ~/onevpl-gpu \
    && cd ~/onevpl-gpu \
    && mkdir build && cd build \
    && cmake .. \
    && make -j$(($(nproc)-1)) \
    && make install
# Compile ffmpeg with only a couple enabled features
RUN cd ~/ffmpeg_sources \
    && wget -O ffmpeg-snapshot.tar.bz2 https://ffmpeg.org/releases/ffmpeg-snapshot.tar.bz2 \
    && tar xjvf ffmpeg-snapshot.tar.bz2 \
    && cd ffmpeg \
    && PATH="$HOME/bin:$PATH" PKG_CONFIG_PATH="$HOME/ffmpeg_build/lib/pkgconfig" ./configure \
        --prefix="$HOME/ffmpeg_build" \
        --pkg-config-flags="--static" \
        --extra-cflags="-I$HOME/ffmpeg_build/include" \
        --extra-ldflags="-L$HOME/ffmpeg_build/lib" \
        --extra-libs="-lpthread -lm" \
        --ld="g++" \
        --bindir="$HOME/bin" \
        --enable-gpl \
        --enable-nonfree \
        --enable-libfdk-aac \
        --enable-libx264 \
        --enable-libx265 \
        --enable-vaapi \
        --enable-libvpl \
        --enable-libvmaf \
        --enable-gnutls \
        --enable-libass \
        --enable-libfreetype \
        --enable-static \
        --disable-shared \
        --enable-pic \
    && PATH="$HOME/bin:$PATH" make -j$(($(nproc)-1)) \
    && make install \
    && hash -r
WORKDIR /makemkv
RUN apt-get update -qq \
    && apt-get install -y \
        build-essential \
        pkg-config \
        libc6-dev \
        libssl-dev \
        libexpat1-dev \
        libgl1-mesa-dev \
        qtbase5-dev \
        zlib1g-dev \
        less
# Patch to fix makemkv and ffmpeg incompatibilities. https://forum.makemkv.com/forum/viewtopic.php?f=3&t=37772
COPY makemkv-oss-1.18.1-ffabi.c.patch /tmp/
RUN wget https://www.makemkv.com/download/makemkv-bin-1.18.1.tar.gz \
    && tar xzf makemkv-bin-1.18.1.tar.gz \
    && wget https://www.makemkv.com/download/makemkv-oss-1.18.1.tar.gz \
    && tar xzf makemkv-oss-1.18.1.tar.gz \
    && cd makemkv-oss-1.18.1 \
    && patch libffabi/src/ffabi.c /tmp/makemkv-oss-1.18.1-ffabi.c.patch && rm /tmp/makemkv-oss-1.18.1-ffabi.c.patch \
    && PKG_CONFIG_PATH=/root/ffmpeg_build/lib/pkgconfig ./configure \
    && make -j$(($(nproc)-1)) && make install \
    && cd .. \
    && cd makemkv-bin-1.18.1 \
    && mkdir tmp && echo accepted > tmp/eula_accepted \
    && make -j$(($(nproc)-1)) && make install
ENTRYPOINT [ "/usr/bin/makemkvcon" ]
CMD [ "-h" ]
