#!/bin/sh
# 1) aliyun mirror
sed 's/archive.ubuntu.com/mirrors.aliyun.com/g' /etc/apt/sources.list -i

# 2) ustc mirror
cat <<EOF > ~/.cargo/config
[source.crates-io]
registry = "https://github.com/rust-lang/crates.io-index"
replace-with = "ustc"
[source.ustc]
registry = "https://mirrors.ustc.edu.cn/crates.io-index"
EOF
