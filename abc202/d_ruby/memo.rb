# https://atcoder.jp/contests/abc200/tasks/abc202_d

# 階乗の計算
N = 60
FACT_MEMO = Array.new(N + 1)
FACT_MEMO[0] = 1
def fact(n)
  return FACT_MEMO[n] unless FACT_MEMO[n].nil?

  FACT_MEMO[n] = n * fact(n - 1)
end

# nCn までのコンビネーションのテーブルを事前に作成 (パスカルの原理)
N = 60
c = Array.new(N + 1) { [0] * (N + 1) }
c[0][0] = 1
0.upto(N - 1).each do |i|
  0.upto(i).each do |j|
    c[i + 1][j] += c[i][j]
    c[i + 1][j + 1] += c[i][j]
  end
end

puts fact(5)
