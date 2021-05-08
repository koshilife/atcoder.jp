# https://atcoder.jp/contests/abc200/tasks/abc200_c

def main
  gets.chomp.to_i
  an = gets.chomp.split(' ').map(&:to_i)
  remainder_dic = {}
  an.each_with_index do |v, i|
    reminder = v % 200
    remainder_dic[reminder] ||= []
    remainder_dic[reminder] << i
  end
  ans = 0
  remainder_dic.each do |_v, indexes|
    len = indexes.length
    ans += len * (len - 1) / 2
  end
  print ans
end

main
