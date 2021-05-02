require 'set'

# 公式解説を参考にポイント：
# - 単純な総当りだとコンビネーション3000 C 3 で計算量は約4.5 * 10 **9 となり終わらない。
# - チームの総合力がx以上となる条件を満たす判定を二分探索に帰着させる。
#   - 各メンバーの各スキルがx以上かどうかの真偽値を0、1と表現すると、
#     そのパターンは全部で32パターンに丸めることができる。
#   - そのうちの３つの組み合わせは最大 コンビネーション 32 C 3 となる。
#   - あとは 1 ~ 10**9 の範囲で2分探索を行う。
#   => 計算量 (10**3)Log(10 **9)となり、実行時間を満たす計算量になる。

def satisfy?(x)
  s = Set.new
  A.each do |values|
    bit = values.map { |v| v >= x ? '1' : 0 }.join('')
    s.add(bit)
  end
  s.to_a.combination([s.length, 3].min).each do |bits|
    result = bits.inject(0) { |sum, bit| sum | bit.to_i(2) }
    return true if result == 31 # '11111'.to_i(2)
  end
  false
end

N = gets.chomp.to_i
A = []
N.times do
  A << gets.chomp.split(' ').map(&:to_i)
end

min = 0
max = 10**9 + 1
loop do
  x = ((min + max) / 2).to_i
  if satisfy?(x)
    min = x
  else
    max = x
  end
  break if (max - min) <= 1
end
print min
