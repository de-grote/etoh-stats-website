# Python scripts for collection Tower data

Since I cannot be bothered manually inputting all the information, I made some python scripts to do it for me!

Because this involves a lot of api calls I store intermediate results in text files, (which I committed since they should be consistent between updates (I hope)).

### get_badges.py

Calls the roblox api to get all badges from the [new](https://www.roblox.com/games/8562822414/Eternal-Towers-of-Hell) and [old](https://www.roblox.com/games/2919924313/MIGRATED-Jukes-Towers-of-Hell) place.

It stores the badge id with the name of the badge.

Does require the api_key file that the main program also needs.

```bash
python get_badges.py etoh > etoh.txt
python get_badges.py legacy > legacy.txt
```

### get_difficulty.py

Calls the Google Sheets api to get all the information from the [Official Difficulty Spreadsheet](https://docs.google.com/spreadsheets/d/1pCnM7Hg-A7MMrNRhao20D7hoIbxZE614yv51LKYfOeA).

It stores the acronym of the tower, difficulty, and background color of the first row, which corresponds to which area the tower is in (almost).

You need credentials.json from the google cloud api, which must have google docs permissions.

You also need to install this: ``pip install gspread google-auth google-api-python-client``

This file was basically just made by chatgpt and I didn't really touch it lol.

```bash
python get_difficulty.py > difficulty.txt
```

### make_queries.py

Makes queries from the previous text files.

These can be used directly on the database, even while the server is running.

This file will need some slight updates everytime a new area is released.

```bash
python make_queries.py > queries.sql
```

### execure queries in db

To execute the queries run

```bash
sqlite3 ../etoh.db < queries.sql
```
