# python3 generate.py <size> > <output_file>

import sys
import random
import string

for i in range(1, 7):
  f = open("input/in-" + str(i) + ".txt", "w")
  f.write(''.join(random.choices(string.ascii_lowercase, k=10**i)))
  f.close()