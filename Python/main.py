import qrcode

data = input("Enter url to be converted to QR code: ")

qr = qrcode.make(data)

qr.save("qr.png")