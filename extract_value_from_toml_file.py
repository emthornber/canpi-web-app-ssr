# Extracts key value from a Cargo.toml file.
#
#   21 February 2021 - E M Thornber
#   Created
#
import argparse
import sys
import tomlkit

def load_cargo_toml(file_path):
    try:
        with open(file_path, 'rb') as toml:
            toml_dict = tomlkit.load(toml)
        
    except tomlkit.TOMLDecodeError as e:
        print(f"Error decoding TOML: {e}", file=sys.stderr)
    except FileNotFoundError as e:
        print(f"Cargo.toml not found: {e}", file=sys.stderr)
    except Exception:
        print(f"System error: {e}", file=sys.stderr)
    
    return toml_dict

if __name__ == "__main__":
    cargo_toml_path = "./Cargo.toml"
    # Initialise Parser
    parser = argparse.ArgumentParser(
        prog="extract_value_from_toml_file",
        description="Extracts key value from a Cargo.toml file.")
    # Add arguments for section and key
    parser.add_argument("-s", "--section", default="package", help="The section to extract from the toml file( default: %(default)s).")
    parser.add_argument("-k", "--key", choices=["name", "version"], help="The key to extract from section in the toml file.")
    # Parse command line
    args = parser.parse_args()
    dict = load_cargo_toml(cargo_toml_path)
    value = dict[args.section][args.key]
    print(value, end='')
