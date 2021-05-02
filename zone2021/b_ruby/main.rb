n, d, h = gets.chomp(' ').split(' ').map(&:to_i)

# puts "N:#{n}, d:#{d}, h:#{h}"
candidates = [0]
n.times do |_i|
  di, hi = gets.chomp.split(' ').map(&:to_f)
  a = (h.to_f - hi) / (d.to_f - di)
  candi = hi - di * a
  candidates << candi
  # puts "  - (di, hi):(#{di}, #{hi})"
end
print candidates.max
