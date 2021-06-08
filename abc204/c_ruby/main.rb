# https://atcoder.jp/contests/abc204/tasks/abc204_c
# 公式解説を参考に実装

n, m = gets.chomp.split(' ').map(&:to_i)

# roads_dic[i] は都市iから道路で直接繋がっている都市のリスト
ROADS_DIC = Array.new(n)
m.times {|i|
  a, b = gets.chomp.split(' ').map(&:to_i)
  ROADS_DIC[a-1] ||= []
  ROADS_DIC[a-1] << b-1
}

def dfs(j, visited_info)
  return if visited_info[j]
  visited_info[j] = true
  roads = ROADS_DIC[j]
  return if roads.nil? || roads.empty?
  roads.each {|k| dfs(k, visited_info)}
end

ans=0
(0...n).each  {|i|
  tmp = [false] * n
  dfs(i, tmp)
  ans += tmp.count {|v| v}
}
puts ans

