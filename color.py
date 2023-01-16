a1 = 0xAA
a2 = 0x00
b1 = 0xAA
b2 = 0x00
c1 = 0xFF
c2 = 0xD5
print(b1, b2, c1, c2)
da = (a2 - a1) / 10
db = (b2 - b1) / 10
dc = (c2 - c1) / 10

for i in range(10):
    aa = int(a1 + i * da)
    bb = int(b1 + i * db)
    cc = int(c1 + dc * i)
    print(f'"#{aa:x}{bb:x}{cc:x}",')
