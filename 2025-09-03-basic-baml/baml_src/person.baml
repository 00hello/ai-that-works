class Person {
  name string
  age int
  email string?
}

function ExtractPerson(text: string) -> Person {
  client CustomGPT4oMini
  prompt #"
    Extract person information from this text:
    
    {{ text }}
    
    {{ ctx.output_format }}
  "#
}

test TestExtractPerson {
  functions [ExtractPerson]
  args {
    text "John Doe is 30 years old and his email is john@example.com"
  }
}