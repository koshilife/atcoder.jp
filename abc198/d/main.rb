# https://atcoder.jp/contests/abc198/tasks/abc198_d
# 覆面算は、0から9の数字がそれぞれに対応する別の記号に置き換えられた計算式を与えられ、
# どの記号が何の数字に対応しているかを推理し、完全な計算式を導き出すパズルである。

def main
  s1 = gets.chomp
  s2 = gets.chomp
  s3 = gets.chomp
  n1_min, n1_max = get_range(s1)
  n2_min, n2_max = get_range(s2)
  n3_min, n3_max = get_range(s3)

  (n1_min..n1_max).each do |n1|
    n2_start = (n3_min - n1)
    next if n2_start > n2_max

    n2_start = n2_min if n2_start < n2_min
    (n2_start..n2_max).each do |n2|
      n3 = n1 + n2
      puts " - 探索中 n1:#{n1}, n2:#{n2}, n3:#{n3}"
      next if n3 < n3_min
      break if n3 > n3_max
      # s1 x s2 検証
      break unless valid_rule3(s1, s2, n1, n2)
      # s1 x s3 検証
      next unless valid_rule3(s1, s3, n1, n3)
      # s2 x s3 検証
      next unless valid_rule3(s2, s3, n2, n3)

      return [n1, n2, n3]
    end
  end
  nil
end

def valid_rule3(str1, str2, n1, n2)
  s1_len = str1.length
  s2_len = str2.length
  (0...s1_len).each do |i|
    break if s2_len < i

    is_same_number = n1.to_s[i] == n2.to_s[i]
    if str1[i] == str2[i]
      return false unless is_same_number
    elsif is_same_number
      return false
    end
  end
  true
end

def get_range(str)
  min = 10**(str.length - 1)
  max = 10**str.length - 1
  [min, max]
end

result = main
if result.nil?
  print 'UNSOLVABLE'
else
  print result.join("\n")
end
