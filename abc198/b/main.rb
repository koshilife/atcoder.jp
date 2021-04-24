# https://atcoder.jp/contests/abc198/tasks/abc198_b

def kaibun?(str)
  len = str.length
  return true if len == 1

  harf_len = len / 2
  front = str[0, harf_len]
  back = str[len.odd? ? harf_len + 1 : harf_len, harf_len]
  front == back.reverse
end

def main
  raw = gets.chomp
  return 'Yes' if kaibun?(raw)

  str = raw.dup
  0.upto(raw.length) do |i|
    return 'No' if raw.reverse[i] != '0'

    str = "0#{str}"
    return 'Yes' if kaibun?(str)
  end
  'No'
end

print main
