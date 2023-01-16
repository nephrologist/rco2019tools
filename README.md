# rco2019tools

適当です。

## visualizer(Rust)

visualizerの使用方法
`cargo run --release --bin vis <INPUT_FILEPATH> <OUTPUT_FILEPATH>`
で出来ます。
`src/bin/vis.rs`の上の方にある `is_visualizeall`がtrueだと10枚区切りで出力されます。
falseにしておけば最終盤面のみ出力されます。
平時のAHC visualizer的な動きを期待して作っていますがポンコツです。
具体的には点数が出ないような入力でも点数をつけてしまう可能性があります。
(一応気をつけてはいますが)

## makemoive.py

`python3 makemovie.py`で、動画が作れます。
cairosvgが必要なので、入っていないとエラーメッセージが出るようになっています。
svg -> pngを担当
`./memo`に`outxxxx.svg`を移動
`./memo`に`outxxxx.png`をつくる
をやってくれます。
最後にコメントアウトしてある魔法の言葉をつぶやくと
`out.mp4`のカタチで動画になります。

### generateinput.py

`./in`, `./out`を作って、inputを`./in`の中につくるアレ

### color.py

グラデーション作るのに使いました。
10段階なのに10分割しているのは笑ってください。
(9分割で良い)
