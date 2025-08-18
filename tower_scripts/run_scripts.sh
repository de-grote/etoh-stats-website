python get_badges.py etoh > etoh.txt
python get_badges.py legacy > legacy.txt
python get_difficulty.py > difficulty.txt
python make_queries.py > queries.sql
sqlite3 ../etoh.db < queries.sql