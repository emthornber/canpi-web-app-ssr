import re
import sys

def extract_arch_from_config_toml(file_path):
    arch_string = "unknown"
    try:
        with open(file_path, 'r') as file:
            content = file.read()
        
        # Use a regular expression to find the arch string
        match = re.search(r'linker\s*=\s*"([^"]+)"', content)
        if match:
            triple = match.group(1)
            architecture = re.match(r'([^-]+)-', triple)
            if re.match(r'^arm$', architecture.group(1)):
                arch_string = "armhf"
            elif re.match(r'^aarch64$', architecture.group(1)):
                arch_string = "arm64"

    except FileNotFoundError as e:
        print(f"config.toml not found: {e}", file=sys.stderr)
    except Exception:
        print(f"System error: {e}", file=sys.stderr)
    
    return arch_string

if __name__ == "__main__":
    config_toml_path = "./.cargo/config.toml"
    arch = extract_arch_from_config_toml(config_toml_path)
    print(arch, end='')
