import json

def readJsonFile(fileName: str):
    with open(fileName, "r") as inFile:
        return json.loads(inFile.read())
