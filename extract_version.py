import re
import sys

def extract_version_from_cargo_toml(file_path):
    version_string = "0.0.0"
    try:
        with open(file_path, 'r') as file:
            content = file.read()
        
        # Use a regular expression to find the version string
        match = re.search(r'version\s*=\s*"([^"]+)"', content)
        if match:
            version_string = match.group(1)

    except FileNotFoundError as e:
        print(f"Cargo.toml not found: {e}", file=sys.stderr)
    except Exception:
        print(f"System error: {e}", file=sys.stderr)
    
    return version_string

if __name__ == "__main__":
    cargo_toml_path = "./Cargo.toml"
    version = extract_version_from_cargo_toml(cargo_toml_path)
    print(version, end='')
