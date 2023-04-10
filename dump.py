from statistics import mean

from collections import Counter

with open('docker_dump', 'r') as file:
   lines = file.readlines()

   new_lines = []
   times = []

   for line in lines:
      splited = line.split('): ')
      new_lines.append(splited[0] + ')')
      times.append(int(splited[1].replace('s', '')))


line_counts = Counter(new_lines)

for line, count in line_counts.items():
   print(f'{count}: {line.strip()}')


print(len(line_counts.items()))
print("average time: ", mean(times))