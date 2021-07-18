# 解説を参考に実装
#
# ポイント:
# - 出題の意図として、 計算量 O(H*W) を目指し、
# - コスト計算式の絶対値を外す前提を置く。
# - 左上から右下の逆パターンを上下反転する工夫。
# - dpテーブルは累積minで定義

def main
  inf = 10**15
  h, w, c = gets.chomp.split(' ').map(&:to_i)
  an = []
  1.upto(h) do |_i|
    an << gets.chomp.split(' ').map(&:to_i)
  end
  ans = inf
  2.times do
    dp = [[inf] * w].dup * h
    0.upto(h - 1).each do |i|
      0.upto(w - 1).each do |j|
        dp[i][j] = [dp[i][j], dp[i - 1][j]].min if i.positive?
        dp[i][j] = [dp[i][j], dp[i][j - 1]].min if j.positive?
        ans = [ans, an[i][j] + (i + j) * c + dp[i][j]].min
        dp[i][j] = [dp[i][j], an[i][j] - (i + j) * c].min
      end
    end
    an.reverse!
  end
  ans
end

puts main
