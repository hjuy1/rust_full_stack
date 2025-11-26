import requests

js = {"id": 6, "teacher_id": 2, "name": "English"}

res = requests.delete("http://localhost:8080/course/1/2", json=js)
print(res.text)
