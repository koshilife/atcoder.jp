def main
  n = gets.chomp.to_i

  data = []
  n.times do
    row = []
    gets.chomp.split('').each do |s|
      row << (s == '#' ? 1 : 0)
    end
    data << row
  end
  data = compress(data)

  goal = []
  n.times do
    row = []
    gets.chomp.split('').each do |s|
      row << (s == '#' ? 1 : 0)
    end
    goal << row
  end
  goal = compress(goal)

  return 'Yes' if goal == data

  3.times.each do
    data = rotate90(data)
    return 'Yes' if goal == data
  end
  'No'
end

def dump(data)
  data.each_with_index do |row, i|
    puts "<row:#{i}> #{row}"
  end
end

def compress(data)
  xn = data.length
  yn = data[0].length

  start_x = 0
  end_x = xn - 1
  start_y = 0
  end_y = yn - 1

  # 行の圧縮 (先頭から探索)
  xn.times do |x|
    is_break = false
    yn.times do |y|
      if data[x][y] == 1
        is_break = true
        break
      end
    end
    break if is_break

    start_x = x + 1
  end

  # 行の圧縮 (後ろから探索)
  (xn - 1).downto(0) do |x|
    is_break = false
    yn.times do |y|
      if data[x][y] == 1
        is_break = true
        break
      end
    end
    break if is_break

    end_x = x - 1
  end

  # 列の圧縮 (先頭から探索)
  yn.times do |y|
    is_break = false
    xn.times do |x|
      if data[x][y] == 1
        is_break = true
        break
      end
    end
    break if is_break

    start_y = y + 1
  end

  # 列の圧縮 (後方から探索)
  (yn - 1).downto(0) do |y|
    is_break = false
    xn.times do |x|
      if data[x][y] == 1
        is_break = true
        break
      end
    end
    break if is_break

    end_y = y - 1
  end

  n_data = []
  (start_x..end_x).each do |x|
    row = []
    (start_y..end_y).each do |y|
      row << data[x][y]
    end
    n_data << row
  end
  n_data
end

def rotate90(data)
  xn = data.length
  yn = data[0].length

  ret = []
  yn.times do |i|
    row = []
    xn.times do |j|
      row << data[xn - 1 - j][i]
    end
    ret << row
  end
  ret
end

puts main
