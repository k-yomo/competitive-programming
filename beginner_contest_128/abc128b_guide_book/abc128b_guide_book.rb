n = gets.strip.split.map(&:to_i).first

rests = []
n.times do |i|
  r = gets.strip.split
  rests << {
    id: i + 1,
    name: r[0],
    score: r[1].to_i
  }
end

puts rests.sort_by { |r| [r[:name], -r[:score]] }.map { |r| r[:id] }.join("\n")
