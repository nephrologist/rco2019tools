import os
from pathlib import Path

hoge = sorted(list(Path("./memo").glob("*")))

for i, path in enumerate(hoge):
    path.rename(path.parent / f"out{i:04}.png")


# for i in range(501):
#     if i % 2 == 0:
#         os.system(f"rm ./memo/out{i:04}.png")
