def main
  alphabets = ('a'..'z').to_a
  alphabet_dic = {}
  gets.chomp.split('').each_with_index { |s, i| alphabet_dic[s] = alphabets[i] }
  n = gets.chomp.to_i
  items = []
  n.times do
    items << gets.chomp
  end
  items.sort_by { |item| item.split('').map { |s| alphabet_dic[s] }.join('') }
end

puts main
