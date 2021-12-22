import cv2 
import pytesseract
import numpy as np
from image_correction import get_grayscale, opening

class WordClassifier:
    def __init__(self):
        self.config = r'--oem 3 --psm 6'

    def ProcessImage(self, image):
        image.string_image = pytesseract.image_to_string(image.image, config=self.config)

    def ProccessImageOpening(self, image):
        gray = get_grayscale(image.image)
        open = opening(gray)
        image.string_image = pytesseract.image_to_string(opening, config=self.config)