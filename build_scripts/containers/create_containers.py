import common
import sys
import os
import stop_containers

containers = common.readJsonFile("containers.json")["containers"]

processed_argv = list()

# Arg defaults
dockerfile = "./Compile_Dockerfile"
releaseMode = False

# Arg processing
for arg in sys.argv[1:]:
    if arg == "--minimal" or arg == "-m":
        dockerfile = "./Minimal_Dockerfile"
    elif arg == "--release" or arg == "-r":
        releaseMode = True
    else:
        processed_argv.append(arg)


additionalArgs = " "
additionalArgs.join(processed_argv)


if len(additionalArgs) > 0:
    additionalArgs += " "

stop_containers.stop_and_remove_containers()

for i in containers:
    os.system(f"docker image rm {i['imageTag']}")

# Build
for i in containers:
    if dockerfile == "./Minimal_Dockerfile": # Compile project locally when minimal option is active
        oldDir = os.getcwd()
        os.chdir(i['directory'])
        buildCommand = f"cargo build { '--release' if releaseMode else '' }"
        print(f"{os.getcwd()} $ {buildCommand}")
        assert os.system(buildCommand) == 0
        os.chdir(oldDir)

    commandList = ["docker", "build", "-t", i['imageTag']]
    if releaseMode: commandList.append("--build-arg releaseOrDebug=release")
    commandList.append(f"--build-arg projectName={os.path.basename((i['directory']).rstrip(' /'))}")
    commandList.extend(["-f", dockerfile])
    commandList.extend(additionalArgs)
    commandList.append(i['directory'])

    command = " ".join(commandList)

    print(f"command: {command}")
    assert os.system(command) == 0
