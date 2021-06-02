# https://atcoder.jp/contests/abc202/tasks/abc202_d
# 公式解説を元に作成

def main
  a, b, k = gets.chomp.split(' ').map(&:to_i)

  # コンビネーションのテーブルを事前に作成 (パスカルの原理)
  c = Array.new(60 + 1) { [0] * (60 + 1) }
  c[0][0] = 1
  0.upto(60 - 1).each do |i|
    0.upto(i).each do |j|
      c[i + 1][j] += c[i][j]
      c[i + 1][j + 1] += c[i][j]
    end
  end

  # 先頭文字数を仮に'a'と決めたときの残りの組み合わせ数の範囲かどうかで先頭文字を決めていく。
  ans = ''
  while a + b > 0
    x = 0
    x = c[a + b - 1][a - 1] if a >= 1
    if k <= x
      ans += 'a'
      a -= 1
    else
      ans += 'b'
      b -= 1
      k -= x
    end
  end
  puts ans
end

main
