# OCR Scripts
## Note
These scripts are heavily setup to work the current MTG Card Sorter in development, these will most likely not just drop in and work for others.

## Dependencies
### Linux
```sudo apt install python-pip python3-opencv```
### Python
```pip install opencv-python opencv-python-headless numpy pytesseract picamera python-dotenv```

## Setup
Inside the .env, CARDDATA will need to point to the json file downloaded from the scryfall api that serves as the cardlook up.