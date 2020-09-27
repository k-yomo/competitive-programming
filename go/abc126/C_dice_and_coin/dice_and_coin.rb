# https://atcoder.jp/contests/abc126/tasks/abc126_c
# my code
n, max = gets.strip.split.map(&:to_i)

result = 0.0
n.times do |i|
  score = i + 1
  count = 0
  loop do
    score = score * 2
    count += 1
    break if score >= max
  end
  result += 1 / n * 0.5 ** count
end

puts result

# the better code another participant wrote
N, K = gets.split.map(&:to_i)
ans = 0

(1..N).each do |n|
  rate = Rational(1, N) #=> (1/N)
  while n <= K - 1
    rate *= Rational(1, 2)
    n *= 2
  end
  ans += rate
end

puts ans.to_f
