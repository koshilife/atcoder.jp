require 'set'

def main
  chars = gets.chomp.split('')
  return 'Weak' if Set[*chars].length == 1

  chars.each_with_index do |c, i|
    next if i.zero?

    before = chars[i - 1].to_i
    week_before = (c.to_i - 1) == -1 ? 9 : c.to_i - 1
    next if before == week_before

    return 'Strong'
  end
  'Weak'
end

puts main
