# https://atcoder.jp/contests/abc199/tasks/abc199_c

def main
  n = gets.chomp.to_i
  s = gets.chomp
  q = gets.chomp.to_i
  is_switched = false
  1.upto q do |_i|
    t, a, b = gets.chomp.split(' ').map(&:to_i)
    if t == 1
      a_index = a - 1
      b_index = b - 1
      if is_switched
        a_index = a > n ? a_index - n : a_index + n
        b_index = b > n ? b_index - n : b_index + n
      end
      tmp = s[a_index]
      s[a_index] = s[b_index]
      s[b_index] = tmp
    else
      is_switched = !is_switched
    end
  end
  s = s[n, n] + s[0, n] if is_switched
  print s
end

main
