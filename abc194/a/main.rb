# https://atcoder.jp/contests/abc194/tasks/abc194_a

def main(max)
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
