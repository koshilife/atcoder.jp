STR = 'chokudai'
STR_LENGTH = STR.length
REMINDER_NUM = 10**9 + 7
MEMO = {}

def main
  chars = gets.chomp.split('')
  c_indexes_map = {}
  chars.each_with_index do |c, i|
    c_indexes_map[c] ||= []
    c_indexes_map[c] << i
  end
  return 0 if c_indexes_map.keys.length < STR_LENGTH

  search(-1, 0, c_indexes_map)
end

#
# current_index 選択したSに対するindex
# word_index 'chokudai'の文字目の探索か
# c_indexes_map
#
def search(current_index, word_index, c_indexes_map, current_ans: 1)
  cache = MEMO.dig(current_index, word_index)
  return cache unless cache.nil?

  # 次の文字として取りうるindexパターン
  char = STR[word_index]
  indexes = c_indexes_map[char].select { |i| i > current_index }
  patterns = indexes.length
  # puts "#{__method__} current_i:#{current_index}, word_i:#{word_index}, char:#{char}, patterns:#{patterns}"

  # パターンなし
  return update_memo(current_index, word_index, 0) if patterns.zero?

  # 最後の文字に到達した場合
  return update_memo(current_index, word_index, (current_ans * patterns) % REMINDER_NUM) if STR_LENGTH <= word_index + 1

  ans = 0
  indexes.each do |next_i|
    next_patterns = search(next_i, word_index + 1, c_indexes_map, current_ans: current_ans)
    ans = (ans + (current_ans * next_patterns)) % REMINDER_NUM if next_patterns.positive?
  end
  update_memo(current_index, word_index, ans)
end

def update_memo(current_index, word_index, ans)
  MEMO[current_index] ||= {}
  MEMO[current_index][word_index] = ans
  ans
end

puts main
