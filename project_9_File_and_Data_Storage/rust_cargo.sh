#!/bin/bash

# 定义当前路径
current_path=$(pwd)

# 进入每个文件夹执行 cargo new 命令
for folder in $(ls -d */); do
    folder_name=$(basename "$folder")
    new_folder_name=${folder_name#*_}
    cd "$folder"
    cargo new "$new_folder_name"
    echo "create $new_folder_name"
    cd "$current_path"
done

