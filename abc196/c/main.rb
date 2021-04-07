# 単純な探索
def main(num)
  return 0 if num < 10

  count = 0
  10.upto(num) do |i|
    digit = Math.log10(i).to_i + 1
    next unless digit.even?

    harf_digit = digit / 2
    s = i.to_s
    count += 1 if s[0, harf_digit] == s[harf_digit, harf_digit]
  end
  count
end

# 高速化
def main2(max)
  return [] if max < 10

  max_digit = Math.log10(max).to_i + 1
  left_ranges = 2.upto(max_digit).map do |digit|
    next unless digit.even?

    harf_digit = digit / 2
    _min = 10**(harf_digit - 1)
    _max = 10**harf_digit - 1
    (_min.._max)
  end.compact
  values = []
  left_ranges.each do |left_range|
    _values = left_range.to_a.map do |val|
      "#{val}#{val}".to_i
    end
    values = [*values, *_values]
  end
  values.select { |v| v <= max }
end

n = gets.chomp.to_i
print main2(n).length
