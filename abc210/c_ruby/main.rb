def main
  n, k = gets.chomp.split(' ').map(&:to_i)
  cn = gets.chomp.split(' ').map(&:to_i)
  ans = [0] * (n - k)
  num_dic = {}
  cn[0, k].each do |v|
    num_dic[v] ||= 0
    num_dic[v] += 1
  end
  ans[0] = num_dic.length
  return k if ans[0] == k

  (1..n - k).each do |i|
    old_c = cn[i - 1]
    new_c = cn[i + k - 1]
    if old_c == new_c
      ans[i] = ans[i - 1]
    else
      if num_dic[old_c] == 1
        num_dic.delete(old_c)
      else
        num_dic[old_c] -= 1
      end
      num_dic[new_c] ||= 0
      num_dic[new_c] += 1
      ans[i] = num_dic.length
    end
    break if ans[i] == k
  end
  ans.max
end

puts main
