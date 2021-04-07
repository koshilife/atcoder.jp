# 整数の入力
n, m = gets.chomp.split(' ').map(&:to_i)
lines = []
n.times.each do |_i|
  lines << gets.chomp.split('').map(&:to_i)
end

# 回答パターン
#
# 00
# 01
# 10
# 11

# puts lines[0].join(',')
# .join(',')
# puts lines
# n, m = line.chomp.split(' ').map(&:to_i)
# スペース区切りの整数の入力
# b, c = gets.chomp.split(' ').map(&:to_i)
# # 文字列の入力
# s = gets.chomp
# # 出力
# print("#{a + b + c} #{s}\n")
# puts n
# puts m
