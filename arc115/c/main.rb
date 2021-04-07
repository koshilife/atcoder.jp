#!ruby -nrprime
# 1.upto($_.to_i){|i|p i.prime_division.sum{_2}+1}

require 'prime'
require 'set'

n = gets.chomp.to_i
def main(n)
  result = {}
  n.times.each do |i|
    num = i + 1
    if num == 1
      result[num] = 1
      next
    end
    div = num.prime_division
    used = [1]
    div.each_with_index do |div_set, _j|
      prime, multi = div_set
      multi.times do |k|
        num_key = prime * (k + 1)
        used << result[num_key] if result.key? num_key
      end
    end
    diff = (Set[*(used.min..used.max).to_a] - Set[*used])
    tmp = diff.length == 0 ? used.max + 1 : diff.first
    result[num] = tmp
  end
  result
end

print "#{main(n).values.join(' ')}\n"
