from selenium import webdriver
import time
from selenium.webdriver.chrome.options import Options
from selenium.webdriver.common.by import By
from selenium.webdriver.support.ui import WebDriverWait
from selenium.webdriver.support import expected_conditions as EC
import pyautogui
from PIL import Image
import cv2
import qrcode
import os
from twilio.rest import Client
import base64

# Create a ChromeOptions object to set the window size
# options = Options()
# options.add_argument("--window-size=1920,1080")

# Create a webdriver object to access the web page
# driver = webdriver.Chrome() #(options=options)
# driver.get("https:webauthn.io")

# Take a screenshot every 30 seconds
while True:
    screenshot = pyautogui.screenshot("screenshot4.png")
    print("Screenshot taken")
# Load the QR code image
    img = cv2.imread("screenshot4.png")

# Initialize the QR code detector
    detector = cv2.QRCodeDetector()

# Detect and decode the QR code
    datat, bbox, rectifiedImage = detector.detectAndDecode(img)

# Print the QR code data
    if len(datat) > 0:
        print("QR code data:", datat)
        # Create a link to the QR code image
        link = f'<a href="{datat}"><img src="screenshot4.png" alt="QR code"></a>'
        print(link)
    else:
      print("No QR code found in the image")
      time.sleep(1)
driver.close()
