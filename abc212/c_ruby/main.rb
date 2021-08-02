#
# ２分探索を行います。
#
# @param [Array<Integer>] arr 検索対象(ソート済み)
# @param [Array<Integer>] search_value 検索値
# @return [Array<mix>]
# - index 0: [Integer, nil] 検索Hitした場合、配列のIndexを返却。
#                           検索Hitしない場合、nil を返却。
# - index 1: [Integer, nil] 検索Hitした場合、nil を返却。
#                           検索Hitしない場合、検索値を超える最小の要素を持つIndexを返却。
#                                           検索値を超える要素が存在しない場合は、配列の長さを返却。
#
def binary_search(arr, search_value, lower: nil, upper: nil)
  lower ||= 0
  upper ||= arr.length
  index = ((lower + upper) / 2).to_i

  arr_v  = arr[index]
  return index, nil if search_value == arr_v

  if search_value > arr_v
    return nil, upper if index >= upper || lower == index

    binary_search(arr, search_value, lower: index, upper: upper)
  else
    return nil, lower if lower >= index || upper == index

    binary_search(arr, search_value, lower: lower, upper: index)
  end
end

def main
  n, m = gets.chomp.split(' ').map(&:to_i)
  an = gets.chomp.split(' ').map(&:to_i).sort
  bn = gets.chomp.split(' ').map(&:to_i).sort

  inf = 10**13
  min = inf
  next_lower = 0
  (0...n).each do |i|
    a = an[i]
    hit_index, near_index = binary_search(bn, a, lower: next_lower)

    unless hit_index.nil?
      min = 0
      break
    end

    if m == near_index
      min = [min, (a - bn.last)].min
      next
    end

    if near_index.zero?
      min = [min, (a - bn[0]).abs].min
      next_lower = 0
    else
      min = [min, (a - bn[near_index - 1]).abs, (a - bn[near_index]).abs].min
      next_lower = near_index - 1
    end
  end
  min
end

puts main
