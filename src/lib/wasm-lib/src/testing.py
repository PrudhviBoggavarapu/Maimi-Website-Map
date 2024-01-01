import json
import re

# Load the JSON data from the uploaded file
file_path = "/home/karma/Downloads/updated_branches_with_title.json"

with open(file_path, "r") as file:
    data = json.load(file)

# Generating Rust struct instances using regex
rust_structs = []

for entry in data:
    # Using regex to format string values properly for Rust
    entry = {k: re.sub(r'(?<!\\)"', r"\"", str(v)) for k, v in entry.items()}

    # Creating the Rust struct representation
    rust_struct = "Location {\n"
    rust_struct += f"    name: \"{entry.get('name', '')}\".to_string(),\n"
    rust_struct += f"    address: \"{entry.get('address', '')}\".to_string(),\n"
    rust_struct += f"    zip: {entry.get('zip', '')},\n"
    rust_struct += f"    phone: \"{entry.get('phone', '')}\".to_string(),\n"
    rust_struct += f"    latitude: {entry.get('latitude', 'None')},\n"
    rust_struct += f"    longitude: {entry.get('longitude', 'None')},\n"
    rust_struct += f"    everything_name: \"{entry.get('everything_name', 'None')}\".to_string(),\n"
    rust_struct += "},"

    rust_structs.append(rust_struct)

# Joining all structs into a single string
rust_code = "\n".join(rust_structs)

print(rust_code)
