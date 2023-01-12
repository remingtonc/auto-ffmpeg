#!/usr/bin/env bash
SOURCE_PATH="/datalake/staging/transcode/source"
TARGET_PATH="/datalake/staging/transcode/target"
for VIDEO_FILE_PATH in $SOURCE_PATH/*.mkv; do
        echo "SOURCE: $VIDEO_FILE_PATH"
        VIDEO_FILENAME=${VIDEO_FILE_PATH##*/}
        podman run -it --rm --group-add keep-groups --device=/dev/dri:/dev/dri -v $SOURCE_PATH:/data/source -v $TARGET_PATH:/data/target localhost/ffmpeg-qs -init_hw_device qsv=qsv:hw -hwaccel qsv -filter_hw_device qsv -hwaccel_output_format qsv -i "/data/source/${VIDEO_FILENAME}" -vf 'hwupload=extra_hw_frames=64,format=qsv' -c:v hevc_qsv -preset slow -c:a libfdk_aac -vbr 5 -c:s copy "/data/target/qs_${VIDEO_FILENAME}"
        echo "TARGET: $TARGET_PATH/$VIDEO_FILENAME"
        sleep 10
done