import requests

projects = requests.get("http://127.0.0.1:7878/project").json()
for j in projects:
    if j["name"] == "Anže & Enej":
        url = "http://" + j["ip"] + ":" + str(j["port"]) + "/sequence"
        print(url)
        seqs = requests.get(url).json()
        assert "Arithmetic" in [j["name"] for j in seqs]
        k = 10
        z = 0
        for j in range(100):
            body = {
                "range": {
                    "from": j * 100,
                    "to": (j + 1) * 100,
                    "step": 1,
                },
                "parameters": [z, k],
                "sequences": [
                ],
            }
            r = requests.post(url + "/Arithmetic", json=body)
            # print(r)
            print(r.json())
        break
else:
    print("Matija & Filip not found")
    exit(1)
    exit(1)
