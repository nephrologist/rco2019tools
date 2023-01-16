# rco2019tools

適当です。

## visualizer(Rust)

visualizerの使用方法
`cargo run --release --bin vis <INPUT_FILEPATH> <OUTPUT_FILEPATH>`
で出来ます。
src/bin/vis.rsの上の方にある `is_visualizeall`がtrueだと10枚区切りで出力されます。
falseにしておけば最終盤面のみ出力されます。


## makemoive.py

`python3 makemovie.py`で、動画が作れます。
cairosvgが必要なので、入っていないとエラーメッセージが出るようになっています。
svg -> pngを担当
./memoにoutxxxx.svgを移動
./memoにoutxxxx.pngをつくる
をやってくれます。
最後にコメントアウトしてある魔法の言葉をつぶやくと
out.mp4ノカタチで動画になります。

### generateinput.py

`./in`, `./out`を作って、inputを`./in`の中につくるアレ

### color.py

グラデーション作るのに使いました。
10段階なのに10分割しているのは笑ってください。
(9分割で良い)
