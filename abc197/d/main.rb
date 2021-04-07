# https://atcoder.jp/contests/abc194/tasks/abc197_d

n = gets.chomp.to_i
x0, y0 = gets.chomp.split(' ').map(&:to_f)
xn, yn = gets.chomp.split(' ').map(&:to_f)
x_center = (x0 + xn) / 2
y_center = (y0 + yn) / 2
# print "N:#{n}, x0:#{x0}, y0:#{y0}, xn:#{xn}, yn:#{yn}, x_center:#{x_center}, y_center:#{y_center}\n"

angle = (Math::PI * 2) / n
cos = Math.cos angle
sin = Math.sin angle

# 調整座標
x0_a = x0 - x_center
y0_a = y0 - y_center
# print "x0, y0: #{x0_a}, #{y0_a}\n"

# 反時計周りに座標を回転
x1_a = (cos * x0_a) + (-sin * y0_a)
y1_a = (sin * x0_a) + (cos * y0_a)

# 座標調整を戻す
x1 = x1_a + x_center
y1 = y1_a + y_center

print "#{x1} #{y1}"
