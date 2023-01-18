# https://github.com/udon1206/AHC003/blob/main/tools/run.shを大いに参考にしています。
# Thanks a lot for udon1206-san



# # コンパイル
# cd /Users/satodai/git/atc_rust/rco-contest-2019-qual/
# cargo build --release --bin bb
cd /Users/satodai/git/atc_rust/rco-contest-2019-qual/tools/
# cargo build --release --bin vis

#[st, en] のseed のファイルを処理する．procsはプロセス数，print_errorはxargs のエラー出力表示
st=0
en=999
procs=32
print_error=1
# output生成
f1(){
  python3 ../main.py < ../in/$1.in >../out/$1.out
}
# ビジュアライザに通して，スコアを標準出力として取得する関数
f2(){
  cargo run --release --bin vis ../in/$1.in ../out/$1.out >> score.txt 2> /dev/null 
}
# xargs で関数使うための処理
export -f f1
export -f f2

#以前の処理を削除
rm -f score.txt
rm -rf out
mkdir out
ls 
# option

usage(){
  cat <<EOM
使い方：
  -s : 開始 seed
  -e : 終了 seed
  -P : プロセス数
  -d : 指定でエラー出力なし
ただし，開始 seed から終了 seed までの入力ファイルは tools/in 下に置いておいてください．
EOM

  exit 2
}

while getopts "s:e:P:d" optKey; do
  case "$optKey" in
    s)
      st=${OPTARG}
      ;;
    e)
      en=${OPTARG}
      ;;
    P)
      procs=${OPTARG}
      ;;
    d)
      print_error=0
      ;;
    '-h' | '--help' | *)
      usage
      ;;
  esac
done

# 並列処理
if [ $print_error = 0 ];
then
  seq -f '%04g' $st $en | xargs -n1 -P$procs -I{} bash -c "f1 {}"
  seq -f '%04g' $st $en | xargs -n1 -P$procs -I{} bash -c "f2 {}"
else 
  seq -f '%04g' $st $en | xargs -t -n1 -P$procs -I{} bash -c "f1 {}"
  seq -f '%04g' $st $en | xargs -t -n1 -P$procs -I{} bash -c "f2 {}"
fi
# score.txt に書き込まれたスコアの計算
python3 /Users/satodai/git/atc_rust/rco-contest-2019-qual/tools/evaluate.py