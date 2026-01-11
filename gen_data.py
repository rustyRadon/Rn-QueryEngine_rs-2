import struct
import random

ROWS = 1_000_000

def generate_column(filename, min_val, max_val):
    print(f"Generating {filename}...")
    with open(filename, 'wb') as f:
        for _ in range(ROWS):
            val = random.randint(min_val, max_val)
            f.write(struct.pack('<i', val))

# New: Generate a dictionary file for departments
def generate_dept_metadata(filename):
    # Mapping ID 1-5 to Names
    names = ["Engineering", "Sales", "Marketing", "HR", "Legal"]
    print(f"Generating {filename}...")
    with open(filename, 'wb') as f:
        for name in names:
            # Pad each name to 32 bytes exactly
            padded_name = name.ljust(32).encode('utf-8')
            f.write(padded_name)

generate_column('age.bin', 18, 75)
generate_column('salary.bin', 30000, 150000)
generate_column('dept.bin', 1, 5) # Updated to match 1-5 IDs
generate_dept_metadata('dept_names.bin')
print("Done!")