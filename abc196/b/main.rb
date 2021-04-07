s = gets.chomp
print s.include?('.') ? s.split('.')[0] : s
