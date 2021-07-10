
n = gets.chomp.to_i
items = []
n.times {|i|
  t, l, r = gets.split(' ').map(&:to_f)
  case t
  when 1
    min = l
    max = r
  when 2
    min = l
    max = r - 0.1
  when 3
    min = l + 0.1
    max = r
  when 4
    min = l + 0.1
    max = r - 0.1
  end
  if min <= max
    items << [min, max]
  end
}

len = items.length
sorted_items = items.sort_by {|item| item[0] }

cnt = 0
(0...len).each {|i|
  (i+1...len).each {|j|
    _min1, _max1 = sorted_items[i]
    _min2, _max2 = sorted_items[j]
    if _min1 == _min2
      cnt += 1
    elsif _min2 <= _max1
      cnt += 1
    end
  }
}
print cnt
