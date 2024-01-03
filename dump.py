from collections import Counter
from statistics import mean

with open('docker_dump', 'r') as file:
   lines = file.readlines()

   new_lines = []
   times = []

   for line in lines:
      if '★' not in line:
         continue

      splited = line.split(')')
      print(splited)
      new_lines.append(splited[0] + ')')
      times.append(int(splited[1].replace('s', '').strip()))


line_counts = Counter(new_lines)

for line, count in line_counts.items():
   print(f'{count}: {line.strip()}')


print(len(line_counts.items()))
print("average time: ", mean(times))
