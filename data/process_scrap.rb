require 'csv'

file = ARGV[0]

data = CSV.new(File.read(file))

rows = []

# filter wrapped lines
data.each do |left, right|
  if left == ""
    if right
      if rows[-1][1][-1] == "-" && ('a'..'z').include?(right[0])
        rows[-1][1][-1] = right
      else
        rows[-1][1] += " #{right}"
      end
    end
  else
    rows << [left, right]
  end
end

questions = {}
question = nil

rows.each do |left, right|
  if left[0] == "T"
    question = {:id => left, :question => right, :answers => {}}
    puts 'oops' if questions[left]
    questions[left] = question
  else
    puts 'oops' if question[:answers][left]
    question[:answers][left] = right
  end
end

questions.keys.sort.each do |id|
  question = questions[id]

  puts "\\begin{question}{#{question[:id]}}{#{question[:question]}}"
  
  question[:answers].each do |_, answer|
    puts "\\answer{#{answer}}"
  end

  puts
end
