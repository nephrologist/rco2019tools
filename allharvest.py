with open("./allharvest", "w") as f:
    for i in range(50):
        for j in range(50):
            f.write(f"2 {i} {j}\n")
