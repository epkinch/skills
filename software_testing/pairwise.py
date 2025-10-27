from allpairspy import AllPairs

parameters = [
    ["12KEY", "NOKEYS", "QWERTY", "UNDEFINED"],
    ["DPAD", "NONAV", "TRACKBALL", "UNDEFINED", "WHEEL"],
    ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K"],
    ["LANDSCAPE", "PORTRAIT", "SQUARE", "UNDEFINED"],
    ["MASK", "NO", "UNDEFINED", "YES"],
]

print("PAIRWISE")
for i, pairs in enumerate(AllPairs(parameters)):
    print("{:2d}: {}".format(i, pairs))