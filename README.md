# skyline-messaging
A simple messaging protocol and server for writing your own clients to talk to other people.

## Docs

### Send a message to the chat server
Here's a python example of how to send a message to the chat server in python:
```python
#!/usr/bin/python3

import requests
import sys
import json


url = 'http://localhost:8080/message'
message = {"user":"MY USERNAME", "text":"Hello, World!"}
x = requests.post(url, data = json.dumps(message))
```

It sends a HTTP POST message to the server (here it's at localhost:8080) at `/message`. It sends message data in a JSON format like this:
```
{"user":<USERNAME>,"text":<MESSAGE>}
```

### Receive messages live
Here's an example of how to show a live updating feed of new messages being broadcast from the server:
```python
import requests

r = requests.get('http://localhost:8080/stream_messages', stream=True)

if r.encoding is None:
    r.encoding = 'utf-8'

for line in r.iter_lines(decode_unicode=True):
    if line:
        print(line)
```

Messages are recieved on a bytes stream of messages from `/stream_messages` that look like this:
```
{"text":<MESSAGE>,"user":<NAME>,"time":<TIME IN RFC 3339>}
```
