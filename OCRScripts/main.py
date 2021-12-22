import display_picture
from MTGLibrary import *
from pytesseract import Output
from MTGFactory import Generate_Magic_Card

library = MTGLibrary("A:\\Coding\\MTGSorter\\Data\\default-cards.json")

card = Generate_Magic_Card('A:\\Coding\\MTGSorter\\Images\\image.jpg')

card_search = library.Look_Up_Card(card)
print(card_search)

#gray_img = image_correction.get_grayscale(cropped_image)
##threshimage = image_correction.thresholding(gray_img, 70, 255)

#d = pytesseract.image_to_data(threshimage, output_type=Output.DICT)
#print(d)


#gray = image_correction.get_grayscale(img)


display_picture.ShowPicture(card.image)
