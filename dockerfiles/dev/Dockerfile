FROM theomn/rustup-base:latest

ARG DEVELOPER_UID
ARG DEVELOPER_GID

RUN apt-get update \
    && apt-get install --no-install-recommends -y \
        sudo \
    && rm -rf /var/lib/apt/lists/*
# Make a user and group to match the user running the build
RUN groupadd -g ${DEVELOPER_GID} developer \
    && useradd -u ${DEVELOPER_UID} \
               -g ${DEVELOPER_GID} \
               -G users \
               -G sudo \
               --create-home developer \
    && chown -R developer:developer /rust \
    && echo "developer ALL=(ALL) NOPASSWD:ALL" > /etc/sudoers.d/00-developer

# libs for our app

RUN apt-get update \
    && apt-get install --no-install-recommends -y \
        cmake \
        git \
        libgtk2.0-dev \
        pkg-config \
        libavcodec-dev \
        libavformat-dev \
        libswscale-dev \
        python-dev \
        python-numpy \
        libtbb2 \
        libtbb-dev \
        libjpeg-dev \
        libpng-dev \
        libtiff-dev \
        libdc1394-22-dev \
    && rm -rf /var/lib/apt/lists/*

ENV OPENCV_VERSION=2.4.13.4

WORKDIR /root

RUN echo "Building OpenCV" \
    && VERS=$OPENCV_VERSION \
    && curl -L --output opencv-$VERS.tar.gz https://api.github.com/repos/opencv/opencv/tarball/$VERS \
    && mkdir opencv-$VERS  \
    && tar xvzf opencv-$VERS.tar.gz -C opencv-$VERS --strip-components 1 \
    && cd opencv-$VERS \
        && mkdir build \
        && cd build \
            && cmake -DENABLE_PRECOMPILED_HEADERS=OFF \
                     -DCMAKE_BUILD_TYPE=Release \
                      -DCMAKE_INSTALL_PREFIX=/usr/local .. \
            && make -j7 && make install \
        && cd .. \
    && rm -rf opencv-* \
    && ldconfig

 
# Once all the package/lib installs are complete, configure the local user
# and switch to userland.

# !!! Anything requiring root should happen before this point.
USER developer

VOLUME /code
WORKDIR /code

