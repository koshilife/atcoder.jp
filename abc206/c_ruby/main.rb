n = gets.chomp.to_i
an = gets.split(' ').map(&:to_i)
number_dic = {}
an.each_with_index {|a|
  number_dic[a] ||= 0
  number_dic[a] += 1
}
total = n * (n -1) / 2
number_dic.each {|a, count|
  if count >= 2
    total -= (count * (count -1)) / 2
  end
}
puts total
