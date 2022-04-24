macosClear
#pip3 install yitizi && pip3 install pinyin && python3 generate_header.py && rustc -C opt-level=3 zh.rs
rustc -C opt-level=3 zh.rs
exit 1