echo 'DATABASE_URL = "sqlite://etoh.db"' > .env
echo -n 'PLACE ROBLOX API KEY HERE' > api_key
sqlite3 etoh.db < schema.sql