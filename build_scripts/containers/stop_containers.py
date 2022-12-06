import common
import sys
import os
def stop_and_remove_containers():
    containers = common.readJsonFile("containers.json")["containers"]

    # Stop old containers
    for i in containers:
        os.system(f"docker stop {i['containerName']}")
        os.system(f"docker rm {i['containerName']}")


if __name__ == "__main__":
    stop_and_remove_containers()