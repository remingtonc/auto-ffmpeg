#!/usr/bin/env bash
SOURCE_PATH="/datalake/staging/transcode/source"
TARGET_PATH="/datalake/staging/transcode/target"
for VIDEO_FILE_PATH in $SOURCE_PATH/*.mkv; do
        echo "SOURCE: $VIDEO_FILE_PATH"
        VIDEO_FILENAME=${VIDEO_FILE_PATH##*/}
        podman run -it --rm --group-add keep-groups --device=/dev/dri:/dev/dri -v $SOURCE_PATH:/data/source -v $TARGET_PATH:/data/target --entrypoint=ffmpeg auto-ffmpeg-ffmpeg-qs:latest -vaapi_device /dev/dri/renderD128 -i "/data/source/${VIDEO_FILENAME}" -vf 'format=p010,hwupload' -c:v hevc_vaapi -preset slow -c:a libfdk_aac -vbr 5 -c:s copy "/data/target/vaapi_${VIDEO_FILENAME}"
        echo "TARGET: $TARGET_PATH/$VIDEO_FILENAME"
        sleep 10
done