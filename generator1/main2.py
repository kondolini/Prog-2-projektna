import requests
import json


def get_sequence_info():
    projects = requests.get("http://127.0.0.1:7878/project").json()
    for project in projects:
        if project["name"] == "Anže & Enej":
            url = "http://" + project["ip"] + ":" + str(project["port"]) + "/sequence"
            return url, requests.get(url).json()
    return None, None

def display_sequence_info(seqs):
    print("\nAvailable sequences with descriptions:")
    for seq in seqs:
        print(f"{seq['name']}: {seq['description']}")
    print()

def get_sequence_choice(seqs):
    print("Available sequences:")
    for i, seq in enumerate(seqs):
        print(f"{i+1}: {seq['name']}")
    
    choice = int(input("Choose a sequence by number (or 0 to see sequence info): "))
    
    if choice == 0:
        display_sequence_info(seqs)
        return get_sequence_choice(seqs) 
    
    if choice < 1 or choice > len(seqs):
        print("Invalid choice.")
        return None
    return seqs[choice - 1]


def get_parameters(seq_info):
    parameters = []
    for i in range(seq_info['parameters']):
        param = float(input(f"Enter parameter {i+1}: "))
        parameters.append(param)
    return parameters

def get_nested_sequences(seq_info, seqs):
    nested_sequences = []
    for i in range(seq_info['sequences']):
        print(f"Creating nested sequence {i+1}...")
        nested_seq_info = get_sequence_choice(seqs)
        if not nested_seq_info:
            return None
        nested_parameters = get_parameters(nested_seq_info)
        nested_sequences.append({
            "name": nested_seq_info['name'],
            "parameters": nested_parameters,
            "sequences": get_nested_sequences(nested_seq_info, seqs) if nested_seq_info['sequences'] > 0 else []
        })
    return nested_sequences

def get_range():
    from_value = int(input("Enter range 'from' value: "))
    to_value = int(input("Enter range 'to' value: "))
    step_value = int(input("Enter range 'step' value: "))
    return {"from": from_value, "to": to_value, "step": step_value}

def build_body(seq_info, seqs):
    body = {
        "range": get_range(),
        "parameters": get_parameters(seq_info),
        "sequences": get_nested_sequences(seq_info, seqs)
    }
    return body


def send_request(url, seq_info, body):
    r = requests.post(f"{url}/{seq_info['name']}", json=body)
    if r.status_code == 200:
        print("Response:", r.json())
    else:
        print("Failed to get a valid response:", r.text)


def main():
    url, seqs = get_sequence_info()
    if not url:
        print("Anže & Enej project not found.")
        return

    seq_info = get_sequence_choice(seqs)
    if not seq_info:
        return

    body = build_body(seq_info, seqs)
    send_request(url, seq_info, body)

if __name__ == "__main__":
    main()
