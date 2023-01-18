cargo run --release --bin vis ../in/0000.in ../out/0000.out

/Users/satodai/.pyenv/shims/python3 makemovie.py

ffmpeg -r 10 -i ./memo/out%04d.png -vcodec libx264 -pix_fmt yuv420p -r 10 out.mp4