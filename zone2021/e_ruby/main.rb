# https://atcoder.jp/contests/zone2021/tasks/zone2021_e

raw_x, raw_y = gets.chomp.split(' ').map(&:to_i)
X = raw_x - 1
Y = raw_y - 1

A = []
B = []
raw_x.times do |_i|
  A << gets.chomp.split(' ').map(&:to_i)
end
(raw_x - 1).times do |_i|
  B << gets.chomp.split(' ').map(&:to_i)
end

COST_MAP = { 0 => { 0 => 0 } }
def update_cost_map(x, y, new_cost)
  cost = COST_MAP.dig(x, y)
  if cost.nil?
    COST_MAP[x] ||= {}
    COST_MAP[x][y] = new_cost
    # puts " => update:(#{x},#{y}) cost:#{new_cost}"
    return true
  end

  if cost > new_cost
    COST_MAP[x][y] = new_cost
    # puts " => update:(#{x},#{y}) cost:#{new_cost}, old:#{cost}"
    return true
  end
  false
end

# (x,y) から (dist_x,dist_y) までの最小コストを算出
def search(x, y, dist_x, dist_y, current_cost: 0, before_move: nil)
  # puts "current:(#{x},#{y}), current_cost:#{current_cost}, before_move:#{before_move}"
  return if x > dist_x
  return if y > dist_y

  # 取りうる移動パターン
  move_patterns = []
  move_patterns << :left if x > 0 && (before_move != :left && before_move != :right)
  move_patterns << :right if x < dist_x && before_move != :left
  move_patterns << :down if y > 0 && before_move != :up
  move_patterns << :up if y < dist_y

  # puts " move_patterns:#{move_patterns}"
  return if move_patterns.empty?

  # コスト表更新
  move_patterns.each do |pattern|
    case pattern
    when :left
      1.upto(x) do |i|
        cost = current_cost + 1 + i
        did_update = update_cost_map(x - i, y, cost)
        search(x - i, y, dist_x, dist_y, current_cost: cost, before_move: :left) if did_update
      end
    when :right
      cost = current_cost + B[x][y]
      did_update =  update_cost_map(x + 1, y, cost)
      search(x + 1, y, dist_x, dist_y, current_cost: cost, before_move: :right) if did_update
    when :down
      cost = current_cost + A[x][y - 1]
      did_update = update_cost_map(x, y - 1, cost)
      search(x, y - 1, dist_x, dist_y, current_cost: cost, before_move: :down) if did_update
    when :up
      cost = current_cost + A[x][y]
      did_update = update_cost_map(x, y + 1, cost)
      search(x, y + 1, dist_x, dist_y, current_cost: cost, before_move: :up) if did_update
    end
  end
end

search(0, 0, X, Y)
print COST_MAP[X][Y]
