import struct
import random

ROWS = 1_000_000

def generate_column(filename, min_val, max_val):
    print(f"Generating {filename}...")
    with open(filename, 'wb') as f:
        for _ in range(ROWS):
            # '<i' means: little-endian, 32-bit integer (i32)
            val = random.randint(min_val, max_val)
            f.write(struct.pack('<i', val))

generate_column('age.bin', 18, 75)
generate_column('salary.bin', 30000, 150000)
generate_column('dept.bin', 1, 10)
print("Done! 1,000,000 rows generated.")