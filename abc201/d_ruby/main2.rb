# https://atcoder.jp/contests/abc201/tasks/abc201_d

# v1からの変更点
# 再帰関数を公式解説を元にシンプルにリファクタ
# => 結果はTLE

def main
  h, w = gets.chomp.split(' ').map(&:to_i)
  h_index = h - 1
  w_index = w - 1
  game_map = []
  (0..h_index).each do
    game_map << gets.chomp.split('').map { |c| c == '+' ? 1 : -1 }
  end

  memo = {}
  search = lambda do |i, j|
    memo_key = "#{i}_#{j}"
    return 0 if i == h_index && j == w_index
    return memo[memo_key] if memo.key?(memo_key)

    res = []
    res << game_map[i + 1][j] - search.call(i + 1, j) if i < h_index
    res << game_map[i][j + 1] - search.call(i, j + 1) if j < w_index
    memo[memo_key] = res.max
  end

  result = search.call(0, 0)
  if result.zero?
    'Draw'
  elsif result.positive?
    'Takahashi'
  else
    'Aoki'
  end
end

print main
