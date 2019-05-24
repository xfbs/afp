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

def fix(str)
  s = str.gsub("%", "\\%")

  if s =~ /{|}/
    puts s
  end

  s
end

sections = CSV.new(File.read(ARGV[1])).to_a

questions.keys.sort.each do |id|
  question = questions[id]

  while sections[0] && question[:id][0...sections[0][0].size] == sections[0][0]
    if sections[0][0].size == 2
      puts "\\section{#{fix(sections[0][1])}}"
    else
      puts "\\subsection{#{fix(sections[0][1])}}"
    end
    puts
    sections.shift
  end

  puts "\\begin{question}{#{fix(question[:id])}}{#{fix(question[:question])}}"
  
  question[:answers].each do |_, answer|
    puts "\\answer{#{fix(answer || "")}}"
  end

  puts "\\end{question}"
  puts
end
