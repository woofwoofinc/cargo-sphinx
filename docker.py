#!/usr/bin/env python3

import argparse
import subprocess

IMAGE_NAME = 'cargo-sphinx'


def has_image(name):
    cmd = "docker images | awk '{{print $1}}' | grep '^{name}$' > /dev/null".format(
            name=name),
    proc = subprocess.run(cmd, shell=True)
    return proc.returncode == 0


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('action', nargs='?',
            help="Either 'shell' or 'docs'")

    action = parser.parse_args().action

    if not has_image(IMAGE_NAME):
        run_build(IMAGE_NAME)

    if action == 'shell':
        run_shell(IMAGE_NAME)
    elif action == 'docs':
        run_docs(IMAGE_NAME)
    else:
        print("Unknown action '{}' specified.")

def run_build(image):
    cmd = "docker build -t {name} .".format(name=image)
    subprocess.run(cmd, shell=True, check=True)

def run_shell(image):
    cmd = """docker run -it \\
             -v "$(pwd):/{name}" \\
             --workdir=/{name} \\
             {name} \\
             /bin/bash""".format(name=image)
    subprocess.run(cmd, shell=True)

def run_docs(image):
    cmd = """docker run -it \\
             -v "$(pwd):/{name}" \\
             --workdir=/{name}/docs \\
             {name} \\
             make clean html""".format(name=image)
    subprocess.run(cmd, shell=True)

if __name__ == "__main__":
    main()
