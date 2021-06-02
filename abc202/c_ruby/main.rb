# https://atcoder.jp/contests/abc202/tasks/abc202_c

def main
  _ = gets
  an = gets.chomp.split(' ').map(&:to_i)
  bn = gets.chomp.split(' ').map(&:to_i)
  cn = gets.chomp.split(' ').map(&:to_i)
  ah = {}
  an.each_with_index do |v, i|
    ah[v] ||= []
    ah[v] << i + 1
  end
  bh = {}
  bn.each_with_index do |v, i|
    bh[v] ||= []
    bh[v] << i + 1
  end
  ch = {}
  cn.each_with_index do |v, i|
    ch[v] ||= []
    ch[v] << i + 1
  end

  cnt = 0
  ah.each do |av, a_indexes|
    b_indexes = bh[av]
    next if b_indexes.nil?

    b_indexes.each do |bi|
      c_indexes = ch[bi]
      next if c_indexes.nil?

      cnt += a_indexes.length * c_indexes.length
    end
  end
  puts cnt
end

main
