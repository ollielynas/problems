"""Mendel's First Law"""

dom,hetro,rec = input().split(" ")

dom = int(dom)
hetro = int(hetro)
rec = int(rec)
pop = dom + hetro + rec

total = 0

# 2 dom
pick1 = dom/pop
pick2 = (dom - 1)/(pop - 1)
total += pick1 * pick2

# 1 dom + 1 hetro
pick1 = dom/pop
pick2 = (hetro)/(pop - 1)
total += (pick1 * pick2)

# 1 hetro + 1 dom
pick1 = hetro/pop
pick2 = (dom)/(pop - 1)
total += (pick1 * pick2)

# 1 dom + 1 rec
pick1 = dom/pop
pick2 = (rec)/(pop - 1)
total += (pick1 * pick2)

# 1 rec + 1 dom
pick1 = rec/pop
pick2 = (dom)/(pop - 1)
total += (pick1 * pick2)

# 1 hetro + 1 rec
pick1 = hetro/pop
pick2 = (rec)/(pop - 1)
total += (pick1 * pick2) * 0.5

# 1 rec + 1 het
pick1 = rec/pop
pick2 = (hetro)/(pop - 1)
total += (pick1 * pick2) * 0.5

# 1 hetro + 1 hetro
pick1 = hetro/pop
pick2 = (hetro - 1)/(pop - 1)
total += (pick1 * pick2) * 0.75

print(total)
