import pytesseract
from pytesseract import Output
import cv2
from MTGCard import MTGCard
from image_correction import * 

MAX_VALID_YEAR = 2022
MIN_VALID_YEAR = 1988
TESSERACT_DETECTION_CONFIG = '--psm 3 -l eng'

def Get_Card_From_File(file_location):
    return cv2.imread(file_location)

def Get_Card_From_Bytes(byte_data):
    bytes_in = bytes(byte_data)
    nparr = np.fromstring(bytes_in, np.uint8)
    return cv2.imdecode(nparr, cv2.IMREAD_ANYCOLOR)

def Prep_Card_For_OCR(image):
    rot = detectAngle(image)
    enitre_card = rotate(image, rot)
    entire_card = crop_picture(enitre_card, 0.10, 0.85, 0.30, 0.76)
    entire_card = remove_background(entire_card)
    return entire_card

def Get_Card_Fields(image):
    card_name = crop_picture(image, 0.05, 0.12, 0.06, 0.7)
    bottom_text = crop_picture(image, 0.90, 0.98, 0.06, 0.7)
    main_text = crop_picture(image, 0.63, 0.90, 0.06, 0.96)
    return card_name, main_text, bottom_text

def build_text(data):   
    list(data.keys())
    n_boxes = len(data['level'])
    last_valid_block = -1
    line_num = -1
    text = []

    for i in range(n_boxes):
        if int(data['conf'][i]) > 60:
            temp_text = data['text'][i]
            temp_text = temp_text.replace(" ", "")
            if temp_text == "":
                continue
            if last_valid_block != int(data['block_num'][i]):
                last_valid_block = int(data['block_num'][i])
                line_num = line_num + 1
                text.append("")
            if text[line_num] == "":
                text[line_num] = temp_text
            else:
                text[line_num] = text[line_num] + " " + temp_text
    return text


def Get_Card_Text(image):
    data = pytesseract.image_to_data(image, output_type=Output.DICT)
    text = build_text(data)
    return text

def Determine_Copyright_Year(image):
    valid_years = []
    text = Get_Card_Text(image)
    for str in text:
        tokens = str.split()
        for token in tokens:
            if token.isdigit() and len(token) == 4:
                valid_years.append(token)
    best_guess = 0
    for year in valid_years:
        if int(year) > best_guess and int(year) < MAX_VALID_YEAR and int(year) > MIN_VALID_YEAR:
            best_guess = int(year)

    return best_guess

def Generate_Magic_Card(image):
    # Setup Card for parsing
    image = Prep_Card_For_OCR(image)

    # Get Card Segments
    card_name, main_text, bottom_text = Get_Card_Fields(image)

    card = MTGCard()
    card.image = image
    temp_name = Get_Card_Text(card_name)
    if len(temp_name) > 0:
        card.found = True
        card.card_name = temp_name[0]
        card.copyright = Determine_Copyright_Year(bottom_text)
        card.flavour_text = Get_Card_Text(main_text)
    return card