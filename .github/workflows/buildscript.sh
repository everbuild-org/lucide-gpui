#!/usr/bin/bash

git clone https://github.com/zed-industries/zed.git --branch main --single-branch && \
ZED_COMMIT=$(git -C zed rev-parse HEAD) && \
rm -rf zed && \
cat lucide-gpui/Cargo.toml | sed "s/rev = \".*\"/rev = \"$ZED_COMMIT\"/" | tee lucide-gpui/Cargo.toml && \
cargo update -p lucide-gpui && \
git commit -am "[ci] Sync gpui dependency to zed#$ZED_COMMIT" && \
git push