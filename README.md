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

- 弾けている提出
  - 長さが2500以上あるもの
- 弾けていない提出
  - `a b c`の形でない出力.
  - `a b c d`の形で出力しているファイルもおそらく読めてしまいそうな気がしている

他、実行に時間がかかる。不定の長さのoutput(今回は出力ファイルに出力ファイルの長さがないから困った)の処理がイケていない、という反省点もあります。

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
