# https://atcoder.jp/contests/abc198/tasks/abc198_c

def main
  r, x, y = gets.chomp.split(' ').map(&:to_i)
  distance = Math.sqrt(x**2 + y**2)
  return 1 if distance == r
  return 2 if distance <= 2 * r

  (distance / r).ceil
end

print main
