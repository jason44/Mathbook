import re

# Regular expression for matching a single variable math function
pattern = r'[A-Za-z]+\w*\s*=\s*[-+]?[0-9]*\.?[0-9]*\s*([+\-*/]\s*[-+]?[0-9]*\.?[0-9]*\s*)*'

# Sample input string
input_string = 'x = 3.14 + 2.7 / tan(y) - 5'

# Find all matches of the pattern in the input string
matches = re.findall(pattern, input_string)

# Print the matches
print(matches)

