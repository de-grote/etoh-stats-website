import gspread
from google.oauth2.service_account import Credentials
from googleapiclient.discovery import build

# pip install gspread google-auth google-api-python-client

# Load credentials and connect to the sheet
SCOPES = ['https://www.googleapis.com/auth/spreadsheets.readonly']
SERVICE_ACCOUNT_FILE = 'credentials.json'  # Path to your service account JSON

# Authenticate
creds = Credentials.from_service_account_file(
    SERVICE_ACCOUNT_FILE, scopes=SCOPES)

# Connect to gspread
gc = gspread.authorize(creds)

# Open the spreadsheet and worksheet
spreadsheet_id = '1pCnM7Hg-A7MMrNRhao20D7hoIbxZE614yv51LKYfOeA'
sheet_name = 'Towers'
worksheet = gc.open_by_key(spreadsheet_id).worksheet(sheet_name)

# Read the first and fourth columns
column_1 = worksheet.col_values(1)
column_4 = worksheet.col_values(4)

# Use Google Sheets API for background colors
sheets_api = build('sheets', 'v4', credentials=creds)
sheet_metadata = sheets_api.spreadsheets().get(spreadsheetId=spreadsheet_id).execute()
sheet_id = None
for s in sheet_metadata['sheets']:
    if s['properties']['title'] == sheet_name:
        sheet_id = s['properties']['sheetId']
        break

# Get background color of first column
range_notation = f"{sheet_name}!A1:A{len(column_1)}"
result = sheets_api.spreadsheets().get(
    spreadsheetId=spreadsheet_id,
    ranges=[range_notation],
    fields="sheets(data(rowData(values(userEnteredFormat(backgroundColor)))))"
).execute()

background_colors = []
rows = result['sheets'][0]['data'][0].get('rowData', [])

for row in rows:
    cell = row.get('values', [{}])[0]
    color = cell.get('userEnteredFormat', {}).get('backgroundColor', {})
    # Convert to hex or keep RGB
    rgb = (
        int(color.get('red', 1) * 255),
        int(color.get('green', 1) * 255),
        int(color.get('blue', 1) * 255)
    )
    hex_color = '#{:02X}{:02X}{:02X}'.format(*rgb)
    background_colors.append(hex_color)

# Combine and print results
for i in range(len(column_1)):
    col1_text = column_1[i] if i < len(column_1) else ''
    col4_text = column_4[i] if i < len(column_4) else ''
    bg_color = background_colors[i] if i < len(background_colors) else ''
    print(f"{bg_color} {col1_text} {col4_text}")