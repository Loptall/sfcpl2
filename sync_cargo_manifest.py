import glob
import toml
import argparse

manifest_path = 'Cargo.toml'

parser = argparse.ArgumentParser(description="sync given keys of Cargo.toml")
parser.add_argument('keys', nargs='+')
keys = parser.parse_args().keys

with open(manifest_path, 'r') as f:
    config = toml.load(f)
    for f in glob.glob('**/{}'.format(manifest_path), recursive=True):
        if f != 'Cargo.toml':
            with open(f, 'rt') as r:
                target = toml.load(r)
                for k in keys:
                    target['package'][k] = config['package'][k]
                toml.dump(target, open(f, 'wt'))
