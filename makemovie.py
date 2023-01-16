import os
import sys
from pathlib import Path

OUT_DIR = Path("./memo")
if not OUT_DIR.exists():
    OUT_DIR.mkdir()

# お掃除
os.system("rm ./memo/*")

# out*.svgの一覧
pathlist = sorted(list(Path(".").glob("out*.svg")))
# exit()
for path in pathlist:
    os.system(f"mv {path} ./memo/{path.name}")


pathlist = sorted(list(OUT_DIR.glob("*.svg")))
print(len(pathlist))

try:
    import cairosvg

    # pathlist = [Path("./out0000.svg")]
    if not Path("./memo").exists():
        Path("./memo").mkdir()
    for path in pathlist:
        print(path.name)
        if Path(f"./memo/{path.stem}.png").exists():
            continue
        cairosvg.svg2png(url=str(path), write_to=f"./memo/{path.stem}.png")

    # ffmpeg -r 30 -i ./memo/%04d.png -vcodec libx264 -pix_fmt yuv420p -r 60 out.mp4
except ModuleNotFoundError:
    print("###Error###", file=sys.stderr)
    print("cairosvg is not installed", file=sys.stderr)
    print("please import cairosvg", file=sys.stderr)
    print("command is pip install cairosvg", file=sys.stderr)


# ffmpeg -r 30 -i ./memo/out%04d.png -vcodec libx264 -pix_fmt yuv420p -r 60 out.mp4
