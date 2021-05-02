s = gets.chomp
t = []

is_reverse = false
s.split('').each_with_index do |c, _i|
  if c == 'R'
    is_reverse = !is_reverse
  elsif is_reverse
    if t[0] == c
      t.shift
    else
      t.unshift(c)
    end
  elsif t[t.length - 1] == c
    t.pop
  else
    t.push(c)
  end
end

t = t.reverse if is_reverse

print t.join('')
