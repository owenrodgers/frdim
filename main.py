
def lerp(v0, v1, t):
    return (1 - t) * v0 + t * v1

def main():
    c = lerp(10, 20, 10)
    print(c)

if __name__ == "__main__":
    main()
