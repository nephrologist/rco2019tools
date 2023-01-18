with open("./score.txt", "r") as f:
    memo = []
    for line in f.readlines():
        a, b = line.split()
        memo.append(int(b))


def calcstatistics(A):
    mean = 0
    var = 0
    for a in A:
        mean += a
    if len(A):
        mean /= len(A)
    for a in A:
        var += (a - mean) ** 2
    if len(A):
        var /= len(A)
    return mean, var


mean, var = calcstatistics(memo)

print(f"total score is {sum(memo)}")

print(f"mean is {mean:.2f}")
print(f"var is {var:.2f}")
