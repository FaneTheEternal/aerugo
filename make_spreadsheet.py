import math
import os
import sys
from pathlib import Path

from PIL import Image

dst = Path(sys.argv[1])
frames = [Image.open(dst.joinpath(name)) for name in os.listdir(dst)]
print(f'{len(frames)} frames')
frames = [img.resize((1920, 1080)) for img in frames]

a = 1
for i in range(1, int(math.sqrt(len(frames)) + 1)):
    if len(frames) % i == 0:
        a = i
b = len(frames) // a
a, b = max(a, b), min(a, b)
print(f'{b} x {a}')
assert b <= 8 and a <= 11, 'Too big spreadsheet'
rectangle = [frames[b * i: b * (i + 1)] for i in range(a)]

size = rectangle[0][0].size
print(size)
result = Image.new('RGBA', (size[0] * b, size[1] * a), (0, 0, 0, 0))

for i, row in enumerate(rectangle):
    for j, img in enumerate(row):
        result.paste(img, (size[0] * j, size[1] * i))

result_name = f'assets/{os.path.basename(dst)}.png'
result.save(result_name)
print(f'Result saved in {result_name}')
