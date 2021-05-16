# https://atcoder.jp/contests/abc200/tasks/abc201_d

# メモ化再起で実装
# => 結果はTLE

def main
  h, w = gets.chomp.split(' ').map(&:to_i)
  h_index = h - 1
  w_index = w - 1
  game_map = []
  (0..h_index).each do
    game_map << gets.chomp.split('').map { |s| s == '+' ? 1 : -1 }
  end

  memo_dic = {}
  search = lambda { |i, j, step, own_pt, opponent_pt|
    memo_key = "#{i}-#{j}"
    return memo_dic[memo_key] if memo_dic.key?(memo_key)

    is_own_turn = step.odd?
    current_pt =  memo_key == '0-0' ? 0 : game_map[i][j]
    if is_own_turn
      own_pt += current_pt
    else
      opponent_pt += current_pt
    end

    # puts "Lambda called. i:#{i}, j:#{j}, step:#{step}, own:#{own_pt}, opponent:#{opponent_pt}"

    can_down = i < h_index
    can_right = j < w_index
    return own_pt, opponent_pt if !can_down && !can_right

    next_step = step + 1
    if can_down
      down_own_pt, down_opponent_pt = search.call(i + 1, j, next_step, own_pt, opponent_pt)
      return down_own_pt, down_opponent_pt unless can_right
    end
    if can_right
      right_own_pt, right_opponent_pt = search.call(i, j + 1, next_step, own_pt, opponent_pt)
      return right_own_pt, right_opponent_pt unless can_down
    end

    down_pt = (down_own_pt - down_opponent_pt) * (is_own_turn ? 1 : -1)
    # puts "    - [Down]  down_pt:#{down_pt}, own:#{down_own_pt}, opponent:#{down_opponent_pt}"
    right_pt = (right_own_pt - right_opponent_pt) * (is_own_turn ? 1 : -1)
    # puts "    - [Right] right_pt:#{right_pt}, own:#{right_own_pt}, opponent:#{right_opponent_pt}"
    ret = if down_pt > right_pt
            # puts "  - #{step} Better:(#{i},#{j}) -> (#{i + 1},#{j})"
            [down_own_pt, down_opponent_pt]
          else
            # puts "  - #{step} Better:(#{i},#{j}) -> (#{i},#{j + 1})"
            [right_own_pt, right_opponent_pt]
          end
    memo_dic[memo_key] = ret
    return ret
  }
  own_pt, opponent_pt = search.call(0, 0, 0, 0, 0)
  if own_pt == opponent_pt
    'Draw'
  elsif own_pt > opponent_pt
    'Takahashi'
  else
    'Aoki'
  end
end

print main
