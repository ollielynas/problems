n, k = map(int, input().split())

arr = [0] * k
arr[0] = 1

for i in range(n):
    print(arr)
    arr[i] = sum(arr) - arr[i-1 % k]
print(arr)
