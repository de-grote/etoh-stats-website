area_colors = {
    "#F9E7FF": "Ring 0: Purgatorio",
    "#F3F3F3": "Ring 1: Limbo",
    "#A0D131": "Forgotten Ridge",
    "#666666": "Ring 2: Desire",
    "#9ADE9D": "Garden of Eeshöl",
    "#FFF1CC": "Ring 3: Gluttony",
    "#B7B7B7": "Ring 4: Greed",
    "#0C343D": "Silent Abyss",
    "#0B5394": "Ring 5: Wrath",
    "#237E73": "Lost River",
    "#E69137": "Ring 6: Heresy",
    "#534542": "Ashen Towerworks",
    "#CCFFFF": "Ring 7: Violence",
    "#7DFFFF": "Ring 8: Fraud",
    "#87226E": "The Starlit Archives",
    "#FFFFFF": "Ring 9: Treachery",
    "#4A86E7": "Zone 1: Sea",
    "#04C90C": "Zone 2: Surface",
    "#FFFFFF": "Arcane Area",
    "#9FC5E7": "Zone 3: Sky",
    "#FFFFC7": "Paradise Atoll",
    "#1B4168": "Zone 4: Exosphere",
    "#8298B1": "Zone 5: The Moon",
    "#A627FF": "Zone 6: Mars",
    "#938C85": "Zone 7: Asteroid Belt",
    "#82D4FF": "Zone 8: Pluto",
    "#C58BEA": "Zone 9: Singularity",
    "#674EA7": "Zone 10: Interstellar Shore",
}

# idk if more towers got replaced/renamed
renamed_towers = {
    "ToAB": "ToZD",
    "ToB": "ToVR",
    "ToITI": "ToVF",
    "ToIaOS": "ToFS",
}

def get_key(dic, key):
    dic[key] = dic.get(key, {}) or {}
    return dic[key]

def read_badge_file(towers, file, key):
    with open(file) as badge_file:
        for line in badge_file.readlines():
            s = line.strip().replace("'", "''").split(" ")
            if len(s) < 5 or s[1].lower() != "beat" or s[2].lower() != "the" or "." in line or "?" in line:
                continue
            badge_id = int(s[0])
            tower_name = " ".join(s[3:])
            tower_acronym = "".join(l[0] for l in s[3:])
            if file == "legacy.txt" and tower_acronym in renamed_towers:
                entry = get_key(towers, renamed_towers[tower_acronym])
                entry[key] = badge_id
            else:
                entry = get_key(towers, tower_acronym)
                entry[key] = badge_id
                entry["tower_name"] = tower_name

def all_areas():
    # forgets about ring 9 because duplicate #FFFFFF
    for area in area_colors.values():
        print(f"INSERT OR IGNORE INTO Realm VALUES ('{area}', 0, NULL, NULL);")

def main():
    towers = {}
    read_badge_file(towers, "legacy.txt", "legacy_badge_id")
    read_badge_file(towers, "etoh.txt", "badge_id")
    with open("difficulty.txt") as difficulty:
        difficulty.readline()
        for line in difficulty.readlines():
            color, acronym, diff, *_ = line.split()
            if '?' in diff or '?' in acronym:
                continue
            if color == "#FFFFFF":
                if acronym[0] == 'S' or acronym == "ToIA":
                    realm = "Arcane Area"
                else:
                    realm = "Ring 9: Treachery"
            else:
                realm = area_colors.get(color, None)
            if realm is None:
                continue

            t = get_key(towers, acronym)
            badge_id = t.get("badge_id", None)
            legacy_badge_id = t.get("legacy_badge_id", "NULL")
            tower_name = t.get("tower_name", None)

            if badge_id is None or tower_name is None:
                continue
            
            # in b4 sql injection
            print(f"INSERT OR REPLACE INTO Tower VALUES ({badge_id}, '{tower_name}', '{acronym}', {diff}, '{realm}', {legacy_badge_id});")

if __name__ == "__main__":
    main()
