#!/usr/bin/python3

import requests
import sys
import json


url = 'http://localhost:8080/message'
message = {"user":sys.argv[1], "text":sys.argv[2]}

print(message)

x = requests.post(url, data = json.dumps(message))

print(x.text)