#!/usr/bin/env bash
podman run -it --rm --group-add keep-groups --device=/dev/dri:/dev/dri --device=/dev/cdrom:/dev/cdrom --device=/dev/sg4:/dev/sg4 -v /datalake/staging/transcode/source:/data/source localhost/auto-ffmpeg-makemkv:1.18.1 --robot --decrypt --cache=1024 --minlength=600 mkv disc:0 all /data/source
