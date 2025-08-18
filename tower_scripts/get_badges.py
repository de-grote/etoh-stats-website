import requests

with open("../api_key") as key:
    headers = {
        "x-api-key": key.read()
    }

def get_universe_id(place_id):
    r = requests.get(f"https://apis.roblox.com/universes/v1/places/{place_id}/universe", headers=headers)
    r.raise_for_status()
    return r.json()["universeId"]

def get_badges(universe_id):
    cursor = ""
    while True:
        url = f"https://badges.roblox.com/v1/universes/{universe_id}/badges?limit=100"
        if cursor:
            url += f"&cursor={cursor}"

        response = requests.get(url, headers=headers)
        response.raise_for_status()
        data = response.json()

        for badge in data.get("data", []):
            print(badge["id"], badge["name"].strip())

        cursor = data.get("nextPageCursor")
        if not cursor:
            break

etoh_place = 8562822414
etoh_universe = 3264581003
legacy_place = 2919924313
legacy_universe = 1055653882
if __name__ == "__main__":
    import sys
    if len(sys.argv) == 2:
        if sys.argv[1] == "etoh":
            get_badges(etoh_universe)
        if sys.argv[1] == "legacy":
            get_badges(legacy_universe)
