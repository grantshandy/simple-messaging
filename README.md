# messaging
Temporary name and repository before we decide on a name.

## Docs
Messages are sent to the server through POST on the endpoint `/message` and look like this:
```
{"user":<USERNAME>,"text":<MESSAGE>}
```

Messages are recieved on a bytes stream of messages that look like this:
```
{"text":<MESSAGE>,"user":<NAME>,"time":<TIME IN RFC 3339>}
```
