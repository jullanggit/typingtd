import json

# Replace 'words.txt' with your file path
input_file_path = 'assets/english.txt'
output_file_path = 'assets/english.words.json'

# Read the input file
with open(input_file_path, 'r', encoding='utf-8') as file:
    # Split the file content into a list of words, stripping newlines
    words = [line.strip() for line in file.readlines()]

# Convert the list of words to a JSON format
words_json = json.dumps(words, ensure_ascii=False, indent=4)

# Write the JSON data to the output file
with open(output_file_path, 'w', encoding='utf-8') as file:
    file.write(words_json)

print(f'Words have been successfully converted to JSON and saved in {output_file_path}')
