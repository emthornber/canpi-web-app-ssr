import re

def extract_version_from_cargo_toml(file_path):
    try:
        with open(file_path, 'r') as file:
            content = file.read()
        
        # Use a regular expression to find the version string
        match = re.search(r'version\s*=\s*"([^"]+)"', content)
        
        return match.group(1) if match else ""
    except FileNotFoundError:
        return ""
    except Exception:
        return ""

if __name__ == "__main__":
    cargo_toml_path = "Cargo.toml"
    version = extract_version_from_cargo_toml(cargo_toml_path)
    print(version, end='')
