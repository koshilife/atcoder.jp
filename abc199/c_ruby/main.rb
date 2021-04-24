# https://atcoder.jp/contests/abc199/tasks/abc199_c

def do_query(n, str, query)
  t, a, b = query
  if t == 1
    c1 = str[a - 1]
    c2 = str[b - 1]
    str[a - 1] = c2
    str[b - 1] = c1
    str
  else
    str[n, n] + str[0, n]
  end
end
# puts do_query(3, 'abcdef', [1, 3, 5])
# puts do_query(3, 'abcdef', [2, 2, 2])

def main
  n = gets.chomp.to_i
  s = gets.chomp
  q = gets.chomp.to_i
  # puts "N: #{n}"
  # puts "S: #{s}"
  # puts "Q: #{q}"

  # puts "[0] #{s}"
  1.upto q do |_i|
    query = gets.chomp.split(' ').map(&:to_i)
    s = do_query(n, s, query)
    # puts "[#{_i}] #{s}"
  end

  print s
end

main
