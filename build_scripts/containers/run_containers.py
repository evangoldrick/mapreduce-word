import common
import sys
import os
import stop_containers

containers = common.readJsonFile("containers.json")["containers"]

processed_argv = list()

# Arg defaults

# Arg processing
for arg in sys.argv[1:]:

    processed_argv.append(arg)


additionalArgs = " "
additionalArgs.join(processed_argv)


if len(additionalArgs) > 0:
    additionalArgs += " "
# Stop old containers
stop_containers.stop_and_remove_containers()
# Create new containers
for i in containers:
    command = f"docker run --name {i['containerName']} -d {additionalArgs}{i['imageTag']}"
    print(f"$ {command}")
    assert os.system(command) == 0
