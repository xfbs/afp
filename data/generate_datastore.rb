#!/usr/bin/env ruby
require 'yaml'

sections = ["technik_e"]

def handle(str)
  str
end

datastore = {}
datastore["sections"] = sections.map do |name|
  questions = File.read("#{name}_questions.tex")
  section = {}
  section["name"] = "Technische Kenntnisse der Klasse E"
  section["short"] = "Technik E"
  section["questions"] = []
  section["subsections"] = []
  
  questions.split("\n\n").each do |qsec|
    case qsec
    when /\\section{(.+)}/
      section["subsections"] << {"name" => handle($1), "subsubsections" => []}
    when /\\subsection{(.+)}/
      section["subsections"].last["subsubsections"] << handle($1)
    when /\\begin{question}/
      question = {"answers" => []}
      qsec.split("\n").each do |line|
        case line
        when /\\begin{question}{(.+)}{(.+)}/
          question["id"] = handle($1)
          question["question"] = handle($2)
        when /\\answer{(.*)}/
          question["answers"] << handle($1)
        when /\\end{question}/
        else
          STDERR.puts "Ooops: can't parse this:\n#{line}"
        end
      end
      section["questions"] << question
    else
      STDERR.puts "Ooops: can't parse this:\n#{qsec}"
    end
  end

  section
end

puts datastore.to_yaml
