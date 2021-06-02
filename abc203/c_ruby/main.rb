# https://atcoder.jp/contests/abc203/tasks/abc203_c
# 公式解説を参考に実装

def main
  n, k = gets.chomp.split(' ').map(&:to_i)

  # 村番号毎の所持金追加テーブルを作ります。
  points_dic = {}
  n.times {|i|
    a, b = gets.chomp.split(' ').map(&:to_i)
    points_dic[a] ||= 0
    points_dic[a] += b
  }
  # 友達がいる村を昇順に並べます。
  villages_has_friends = points_dic.keys.sort

  # ループ内容:
  # - 所持金分、村番号を移動します。
  # - 移動分の村で友達がいた村があったか、昇順に並べた「友達いる村リスト」を用いて
  #   追加されるお金を加算して、お金がなくなるまで村移動を繰り返す。
  #
  # この問題のポイント:
  # - 友達の人数が10^5の制約、友達がいる村番号を昇順に並べて、カーソルを管理することで効率よく計算する方法に気づけるかどうか。
  current_village = 0
  next_village_index = 0
  while k > 0
    # puts "current_village:#{current_village}, k:#{k}, next_village_index:#{next_village_index}"
    current_village += k
    k = 0
    loop {
      next_village = villages_has_friends[next_village_index]
      break if next_village.nil? || current_village < next_village
      k += points_dic[next_village]
      next_village_index += 1
    }
  end
  puts current_village
end

main
