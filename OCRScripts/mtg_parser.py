import cv2 

def parse_boxes(data, image):
    list(data.keys())
    n_boxes = len(data['level'])
    for i in range(n_boxes):
        if int(data['conf'][i]) > 0 or True:
            print(data['text'][i])
            (x, y, w, h) = (data['left'][i], data['top'][i], data['width'][i], data['height'][i])
            cv2.rectangle(image, (x, y), (x + w, y + h), (0, 255, 0), 2)
    return image

