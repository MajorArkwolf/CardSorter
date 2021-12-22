import cv2
import numpy as np
import math

def get_grayscale(image):
    return cv2.cvtColor(image, cv2.COLOR_BGR2GRAY)

def remove_noise(image):
    return cv2.medianBlur(image,5)
 
def thresholding(image, threshvalue, maxvalue):
    return cv2.threshold(image, threshvalue, maxvalue, cv2.THRESH_BINARY)[1]

def dilate(image):
    kernel = np.ones((5,5),np.uint8)
    return cv2.dilate(image, kernel, iterations = 1)
    
def erode(image):
    kernel = np.ones((5,5),np.uint8)
    return cv2.erode(image, kernel, iterations = 1)

def opening(image):
    kernel = np.ones((5,5),np.uint8)
    return cv2.morphologyEx(image, cv2.MORPH_OPEN, kernel)

def canny(image):
    return cv2.Canny(image, 100, 200)

def deskew(image):
    coords = np.column_stack(np.where(image > 0))
    angle = cv2.minAreaRect(coords)[-1]
    if angle < -45:
        angle = -(90 + angle)
    else:
        angle = -angle
    (h, w) = image.shape[:2]
    center = (w // 2, h // 2)
    M = cv2.getRotationMatrix2D(center, angle, 1.0)
    rotated = cv2.warpAffine(image, M, (w, h), flags=cv2.INTER_CUBIC, borderMode=cv2.BORDER_REPLICATE)
    return rotated

def match_template(image, template):
    return cv2.matchTemplate(image, template, cv2.TM_CCOEFF_NORMED) 

def crop_picture(img, top_percent, bottom_percent, left_percent, right_percent):
    left = int(img.shape[1] * left_percent)
    right = int(img.shape[1] * right_percent)
    top = int(img.shape[0] * top_percent)
    bottom = int(img.shape[0] * bottom_percent)

    img = img[top:bottom, left: right]
    return img

def rotate(image, angle):
	rows = 0
	cols = 0
	
	if len(image.shape) == 2:
		rows, cols = image.shape
	elif len(image.shape) == 3:
		rows, cols, channels = image.shape
	else:
		return None
		
	M = cv2.getRotationMatrix2D((cols / 2, rows / 2), angle, 1)
	return cv2.warpAffine(image, M, (cols, rows))
	
def detectAngle(image):
	img_before = image.copy()
	img_gray = cv2.cvtColor(img_before, cv2.COLOR_BGR2GRAY)
	img_edges = cv2.Canny(img_gray, 100, 100, apertureSize=3)
	lines = cv2.HoughLinesP(img_edges, 1, math.pi / 180.0, 100, minLineLength=100, maxLineGap=5)
	
	if lines is None or len(lines) == 0:
		return 0

	angles = []
	for x1, y1, x2, y2 in lines[0]:
		cv2.line(img_before, (x1, y1), (x2, y2), (255, 0, 0), 3)
		angle = math.degrees(math.atan2(y2 - y1, x2 - x1))
		angles.append(angle)

	return np.median(angles)

def remove_background(img):
    gray = cv2.cvtColor(img, cv2.COLOR_BGR2GRAY)
    thresh = cv2.threshold(gray, 100, 255, cv2.THRESH_BINARY_INV)[1]

    cnts = cv2.findContours(thresh, cv2.RETR_EXTERNAL, cv2.CHAIN_APPROX_SIMPLE)
    cnts = cnts[0] if len(cnts) == 2 else cnts[1]
    cnts = sorted(cnts, key=cv2.contourArea, reverse=True)

    # Find bounding box and extract ROI
    for c in cnts:
        x,y,w,h = cv2.boundingRect(c)
        if (h > w):
            result_image = img[y:y+h, x:x+w]
            return result_image
    print("Failed to crop card.")