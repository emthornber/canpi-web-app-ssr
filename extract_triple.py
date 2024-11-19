import re
import sys

def extract_triple_from_config_toml(file_path):
    triple_string = "unknown"
    try:
        with open(file_path, 'r') as file:
            content = file.read()
        
        # Use a regular expression to find the triple string
        match = re.search(r'target\s*=\s*"([^"]+)"', content)
        if match:
            triple_string = match.group(1)

    except FileNotFoundError as e:
        print(f"config.toml not found: {e}", file=sys.stderr)
    except Exception:
        print(f"System error: {e}", file=sys.stderr)
    
    return triple_string

if __name__ == "__main__":
    config_toml_path = "./.cargo/config.toml"
    triple = extract_triple_from_config_toml(config_toml_path)
    print(triple, end='')
