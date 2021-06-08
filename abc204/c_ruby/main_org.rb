# https://atcoder.jp/contests/abc204/tasks/abc204_c
# の公式解説を元に、 TLEのコードを修正。

def search(i, roads_dic, visited)
  return if visited[i]
  visited[i] = true
  roads = roads_dic[i]
  return if roads.nil? || roads.empty?
  roads_dic[i].each {|j| search(j, roads_dic, visited) }
end

def main
  n, m = gets.chomp.split(' ').map(&:to_i)

  roads_dic = Array.new(n)
  m.times {|i|
    a, b = gets.chomp.split(' ').map(&:to_i)
    roads_dic[a-1] ||= []
    roads_dic[a-1] << b-1
  }

  total = 0
  (0...n).each do |i|
    tmp = [false] * n
    search(i, roads_dic, tmp)
    total += tmp.count {|v| v}
  end
  puts total
end

main
