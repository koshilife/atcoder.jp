require 'set'

def main
  n = gets.chomp.to_i
  data = {}

  # 同一x座標が2つ以上存在する(辺を作れる) x値
  multi_x_indexes = Set.new

  n.times do |_i|
    x, y = gets.chomp.split(' ').map(&:to_i)
    data[x] ||= {}
    data[x][y] = true
    data[x][:cnt] ||= 0
    data[x][:cnt] += 1
    multi_x_indexes.add(x) if data[x][:cnt] == 2
  end

  cnt = 0
  multi_x_indexes.to_a.combination(2).each do |x1, x2|
    ys = (Set[*data[x1].keys] & Set[*data[x2].keys]).delete(:cnt)
    cnt += ys.to_a.combination(2).count
  end
  cnt
end

puts main
