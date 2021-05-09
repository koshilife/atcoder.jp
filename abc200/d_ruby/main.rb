# https://atcoder.jp/contests/abc200/tasks/abc200_d

# 200の余りパターンは200。
# 鳩ノ巣原理を満たす、N=8で255通りのコンビネーション(255通り)
# なので、N=8以上なら8までで組み合わせを最大 O(201)で求めることが可能
def main
  n = gets.chomp.to_i
  an = gets.chomp.split(' ').map(&:to_i)
  n = [n, 8].min
  mod_dic = []

  # bit 全探索
  (1...2**n).each do |i|
    bits = format("%0#{n}b", i).split('')
    bn_sum = bits.map.with_index { |c, j| c == '1' ? an[j] : nil }.compact.sum % 200
    mod_dic[bn_sum] ||= []
    mod_dic[bn_sum] << bits
    next if mod_dic[bn_sum].length < 2

    # 同一の余りが2件以上あった場合、早期リターン
    puts 'Yes'
    bn = mod_dic[bn_sum][0].map.with_index { |c, j| c == '1' ? j + 1 : nil }.compact
    cn = mod_dic[bn_sum][1].map.with_index { |c, j| c == '1' ? j + 1 : nil }.compact
    puts [bn.length, *bn].join(' ')
    print [cn.length, *cn].join(' ')
    return
  end
  print 'No'
end

main
