PYTHON=/usr/bin/python3

$PYTHON -m pip install yitizi && \
$PYTHON -m pip install pinyin && \
$PYTHON generate_header.py && \
rustc -C opt-level=3 zh.rs

exit 1