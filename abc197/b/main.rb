# https://atcoder.jp/contests/abc194/tasks/abc197_b

h, w, x, y = gets.chomp.split(' ').map(&:to_i)
maps = {}
1.upto(h) do |i|
  line = gets.chomp.split('')
  maps[i] = {}
  1.upto(w) do |j|
    maps[i][j] = line[j - 1]
  end
end
# print maps, x, y
# print "\n h:#{h}, w:#{w}, maps(#{x},#{y}):#{maps[x][y]}\n"

result = 1
# 上下左右に"."が続くまでカウント
x.downto(1) do |i|
  next if x == i
  break if maps[i][y] == '#'

  result += 1
end
x.upto(h) do |i|
  next if x == i
  break if maps[i][y] == '#'

  result += 1
end
y.downto(1) do |j|
  next if y == j
  break if maps[x][j] == '#'

  result += 1
end
y.upto(w) do |j|
  next if y == j
  break if maps[x][j] == '#'

  result += 1
end

print result
