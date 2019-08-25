#!/usr/bin/env bash
SOURCE_PATH="/datalake/staging/transcode/source"
TARGET_PATH="/datalake/staging/transcode/target"
for VIDEO_FILE_PATH in $SOURCE_PATH/*.mkv; do
	echo "SOURCE: $VIDEO_FILE_PATH"
	VIDEO_FILENAME=${VIDEO_FILE_PATH##*/}
	echo "Making first pass on $VIDEO_FILENAME"
	docker run --rm -v $SOURCE_PATH:/data/source -v $TARGET_PATH:/data/target --entrypoint "/bin/bash" ffmpeg -c "ffmpeg -y -i \"/data/source/${VIDEO_FILENAME}\" -c:v libx265 -crf 20 -preset slow -x265-params pass=1 -an -f matroska /dev/null && ffmpeg -i \"/data/source/${VIDEO_FILENAME}\" -c:v libx265 -crf 20 -preset slow -x265-params pass=2 -c:a libfdk_aac -vbr 5 \"/data/target/${VIDEO_FILENAME}\""
	echo "TARGET: $TARGET_PATH/$VIDEO_FILENAME"
done

