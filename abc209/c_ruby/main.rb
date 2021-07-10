#
# <反省>
# 一度、TLEになったことの原因に気づけず、Rustに書き直す判断が無駄だった。
# 大きい数字通しを掛け合わせていたところにRubyの言語仕様で時間がかかっていたと想定される。
# 毎度剰余計算をすることに、気づけたら、少なくとも残り35分はD問題に充てられただろう。
#
# 以下、ACコード

def main
  n = gets.chomp.to_i
  cn = gets.chomp.split(' ').map(&:to_i).sort
  return 0 if cn[n-1] < n
  ans = cn[0]
  1.upto(n-1) do|i|
    ans = (ans * (cn[i] - i)) % (10 ** 9 + 7)
  end
  ans
end
puts main()