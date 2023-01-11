#!/usr/bin/env bash
SOURCE_PATH="/datalake/staging/transcode/source"
TARGET_PATH="/datalake/staging/transcode/target"
for VIDEO_FILE_PATH in $SOURCE_PATH/*.mkv; do
        echo "SOURCE: $VIDEO_FILE_PATH"
        VIDEO_FILENAME=${VIDEO_FILE_PATH##*/}
        sudo podman run -it --rm --device=/dev/dri:/dev/dri -v $SOURCE_PATH:/data/source -v $TARGET_PATH:/data/target --entrypoint=ffmpeg localhost/vaapiffmpeg -vaapi_device /dev/dri/renderD128 -i "/data/source/${VIDEO_FILENAME}" -c:v hevc_vaapi -crf 20 -preset slow -c:a libfdk_aac -vbr 5 -c:s copy "/data/target/qs_${VIDEO_FILENAME}"
        echo "TARGET: $TARGET_PATH/$VIDEO_FILENAME"
        sleep 10
done