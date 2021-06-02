# https://atcoder.jp/contests/abc201/tasks/abc201_d

# v2からの変更点 (TLEを回避するためにやったこと)
# - game_map作る際に文字列比較を文字コードで判定する「+」=「43」: -300ms
# - メモ変数はHashではなく、Arrayでアクセスする形に変更: -300ms
# => 結果 AC

def main
  h, w = gets.chomp.split(' ').map(&:to_i)
  h_index = h - 1
  w_index = w - 1
  game_map = Array.new(h) { gets.chomp.bytes.map { _1 == 43 ? 1 : -1 } }
  memo = Array.new(h) { [nil] * w }

  search = lambda do |i, j|
    return 0 if i == h_index && j == w_index
    return memo[i][j] if memo[i][j]

    res = []
    res << game_map[i + 1][j] - search.call(i + 1, j) if i < h_index
    res << game_map[i][j + 1] - search.call(i, j + 1) if j < w_index
    memo[i][j] = res.max
  end

  %w[Draw Takahashi Aoki][search.call(0, 0) <=> 0]
end

print main
