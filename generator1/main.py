import requests

projects = requests.get("http://127.0.0.1:7878/project").json()
for j in projects:
    if j["name"] == "Anže & Enej":
        url = "http://" + j["ip"] + ":" + str(j["port"]) + "/sequence"
        print(url)
        seqs = requests.get(url).json()
        assert "Geometric" in [j["name"] for j in seqs]
        k = 1
        z = 0.5
        for j in range(10):
            body = {
                "range": {
                    "from": j * 100,
                    "to": (j + 1) * 100,
                    "step": 1,
                },
                "parameters": [2, 0.5],
                "sequences": [
                ],
            }
            r = requests.post(url + "/Geometric", json=body)
            body ={
                "range": {
                    "from": j * 100,
                    "to": (j + 1) * 100,
                    "step": 1,
                },
                "parameters": [2, 0.5],
                "sequences": [
                ],
            }
            # print(r)
            print(r.json())
        break
else:
    print("Matija & Filip not found")
    exit(1)
    exit(1)
