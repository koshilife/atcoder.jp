# https://atcoder.jp/contests/abc200/tasks/abc201_c
require 'set'

def main
  s = gets.chomp
  must = Set.new
  ng_dic = []
  candidates = Set.new
  (0...s.length).each do |i|
    c = s[i]
    case c
    when 'o'
      must.add(i)
    when 'x'
      ng_dic[i] = true
    when '?'
      candidates.add(i)
    end
  end
  if must.length > 4
    print 0
    return
  end

  satisfy = lambda { |num|
    nums = format('%04d', num).split('').map(&:to_i)
    # ng check
    nums.each do |i|
      return false if ng_dic[i]
    end
    # ok check
    (must - Set[*nums]).to_a.length.zero?
  }

  count = 0
  0.upto(9999) do |i|
    count += 1 if satisfy.call(i)
  end
  print count
end

main
