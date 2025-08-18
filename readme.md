# Etoh Stats Website

A website based on [jtoh.info](jtoh.info).

Made in rust using dioxis.

### Setup

Run the provided setup script to generate an .env, api_key file and an empty database.

Make sure to put in your own roblox api key in the api_key file.

```bash
./setup.sh
```

### Running

If you have the dioxis cli installed you can run:

```bash
dx serve
# or
dx serve --platform web
```

### Scripts

In towerscripts/ there are some python scripts to get the newest tower data

For more info, read [the readme of the scripts](tower_scripts/readme.md).