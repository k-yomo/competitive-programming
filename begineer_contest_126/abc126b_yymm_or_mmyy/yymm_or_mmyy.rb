# https://atcoder.jp/contests/abc126/tasks/abc126_b
# my code
s = gets.chomp
x = s[0..1].to_i
y = s[2..3].to_i

if (x > 12 && y > 12) || (x > 12 && y == 0) || (x == 0 && y > 12) || (x == 0 && y == 0)
  puts 'NA'
elsif 0 < x && x <= 12 && 0 < y && y <= 12
  puts 'AMBIGUOUS'
elsif 0 < y && y <= 12
  puts 'YYMM'
elsif x <= 12
  puts 'MMYY'
end

# the better code another participant wrote
s = gets
h = s[0..1].to_i
t = s[2..3].to_i
m = 1..12
case
when (h >= 0 && m.include?(t)) && (t >= 0 && m.include?(h))
  puts 'AMBIGUOUS'
when h >= 0 && m.include?(t)
  puts 'YYMM'
when t >= 0 && m.include?(h)
  puts 'MMYY'
else
  puts 'NA'
end

