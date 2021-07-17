def main
  h,w,c = gets.chomp.split(' ').map(&:to_i)
  an = []
  1.upto(h) {|i|
    wn = gets.chomp.split(' ').map(&:to_i)
    wn.each_with_index {|v,j|
      an << [v,i,j]
    }
  }
  # 移動コストで足切り
  sorted_an = an.sort
  v2,i2,j2 = sorted_an[1]
  max_move_cost = ((h-1) + (w-1)) * c
  reject_v = v2 + max_move_cost
  candidates = sorted_an.reject{|v,i,j| v > reject_v}
  # 残った駅候補で計算
  costs = []
  candidates.combination(2).each {|c1,c2|
    v1,i1,j1 = c1
    v2,i2,j2 = c2
    s_cost = v1 + v2
    m_cost = ((i1-i2).abs() + (j1-j2).abs()) * c
    costs << (s_cost+m_cost)
  }
  costs.min
end

puts main()