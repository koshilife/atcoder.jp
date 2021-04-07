# https://atcoder.jp/contests/abc194/tasks/abc197_c

def calc_or(arr)
  return arr[0] if arr.length == 1

  bit_or = arr[0]
  arr.each_with_index do |v, i|
    next if i == 0

    bit_or |= v
  end
  bit_or
end

n = gets.chomp.to_i
an = gets.chomp.split(' ').map(&:to_i)
l = an.length
result = []
(1...l).each do |i|
  first = an[0, i]
  second = an[i, l - i]
  first_or = calc_or(first)
  second_or = calc_or(second)
  _xor = first_or ^ second_or
  print "[#{i}] xor:#{_xor}, first(#{first_or}):#{first}, second(#{second_or}):#{second}\n"
  result << _xor
end
print result.min
