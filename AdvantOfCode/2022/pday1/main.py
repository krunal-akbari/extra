import time

# this is first part
#takes aproximate 0.002
start = time.time()
inputs = list()
with open('input.txt') as f:
    l = []
    for x in f.readlines():
        if x.strip() != "":
            l.append(int(x.strip()))
        else:
            inputs.append(sum(l))
            l = []

print(max(inputs))
end = time.time()
print(end - start)


#second part
# takes around 0.002
start = time.time()
inputs = list()
with open('input.txt') as f:
    l = []
    for x in f.readlines():
        if x.strip() != "":
            l.append(int(x.strip()))
        else:
            inputs.append(sum(l))
            l = []

inputs.sort()
print(sum(inputs[-3:]))
end = time.time()
print(end - start)
