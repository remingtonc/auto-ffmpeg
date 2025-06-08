#!/usr/bin/env bash
# Current version is 1.18.1
if [[ ! -f keydb.cfg ]]; then
    wget http://fvonline-db.bplaced.net/export/keydb_eng.zip \
    && unzip keydb_eng.zip \
    && rm keydb_eng.zip
fi
if [[ ! -f settings_with_key.conf ]]; then
    cp settings.conf settings_with_key.conf \
    && echo "app_Key = \"`curl -sS https://forum.makemkv.com/forum/viewtopic.php?t=1053 \
    | sed -n 's/.*<code>\([^<]*\)<\/code>.*/\1/p'`\"" >> settings_with_key.conf

fi
podman build --tag auto-ffmpeg-makemkv:1.18.1 .
