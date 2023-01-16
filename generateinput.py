import os
from pathlib import Path
from random import randint, seed

seed(2023013)
INPUT_DIR = Path("./in")
OUTPUT_DIR = Path("./out")
if not INPUT_DIR.exists():
    INPUT_DIR.mkdir()
if not OUTPUT_DIR.exists():
    OUTPUT_DIR.mkdir()

os.system("rm ./in/*.in")
for i in range(50):
    with open(f"./in/{i:04}.in", "w") as f:
        f.write("50 2500\n")
        for _ in range(50):
            temp = [randint(1, 9) for __ in range(50)]
            f.write(" ".join(map(str, temp)) + "\n")
