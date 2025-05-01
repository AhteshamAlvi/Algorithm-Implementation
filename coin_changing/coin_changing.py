def coin_change(C, n, A = []):
    A.append(0)
    
    for i in range(1,n):
        coin_num = float('inf')
        for coin in C:
            if i - coin >= 0:
                coin_num = min(coin_num, 1 + A[i - coin])
        A.append(coin_num)

    return A

def main():
    B = coin_change([1, 5, 10, 25], 100)
    print(B[99])

main()