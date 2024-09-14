import requests

projects = requests.get("http://127.0.0.1:7878/project").json()
for j in projects:
    if j["name"] == "Anže & Enej":
        url = "http://" + j["ip"] + ":" + str(j["port"]) + "/sequence"
        print(url)
        seqs = requests.get(url).json()
        k = 1
        z = 0.5
        for j in range(10):
            body = {
                "range": {
                    "from": j * 10,
                    "to": (j + 1) * 10,
                    "step": 1,
                },
                "parameters": [2, 0.5],
                "sequences": [],
            }
            r = requests.post(url + "/Geometric", json=body)
            body = {
                "range": {
                    "from": j * 10,
                    "to": (j + 1) * 10,
                    "step": 1,
                },
                "parameters": [2, 6],
                "sequences": [
                    {"name": "Geometric", "parameters": [2, 3], "sequences": []},
                    {"name": "Geometric", "parameters": [5, 1], "sequences": []}
                ],
            }
            #r = requests.post(url + "/Lin_Comb", json=body)
            #print(r.json())
            body = {
                "range": {
                    "from": j * 10,
                    "to": (j + 1) * 10,
                    "step": 1,
                },
                "parameters": [2],
                "sequences": [
                    {"name": "Geometric", "parameters": [2, 3], "sequences": []}
                ],
            }
            r = requests.post(url + "/Drop", json=body)
            print(r.json())            
            
        break
else:
    print("Anže & Enej not found")
    exit(1)