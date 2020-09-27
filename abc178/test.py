s = int(input())

def get_integral_value_combination(nums, target):
    def a(idx, l, r, t):
        if t == sum(l): r.append(l)
        elif t < sum(l): return
        for u in range(idx, len(nums)):
            a((u + 1), l + [nums[u]], r, t)
        return r
    return a(0, [], [], target)

print len(get_integral_value_combination(list(range(3, s+1)), s))