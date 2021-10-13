import requests

r = requests.get('http://localhost:8080/stream_messages', stream=True)

if r.encoding is None:
    r.encoding = 'utf-8'

for line in r.iter_lines(decode_unicode=True):
    if line:
        print(line)