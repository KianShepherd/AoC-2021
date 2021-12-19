import matplotlib.pyplot as plt

days = []
p1_runtimes = []
p2_runtimes = []
total_runtimes = []
day = 0

with open('runtimes', 'r') as f:
    for line in f.readlines():
        day = day + 1
        days.append(day)
        times = line.strip().split(" ")
        p1_runtimes.append(float(times[0]))
        p2_runtimes.append(float(times[1]))
        if times[0] == times[1]:
            total_runtimes.append(float(times[0]))
        else:
            total_runtimes.append(float(times[0]) + float(times[1]))

plt.plot(days, p1_runtimes, label="Part 1")
plt.plot(days, p2_runtimes, label="Part 2")
plt.plot(days, total_runtimes, label="Total Runtime")
plt.ylabel('Runtime in miliseconds')
plt.xlabel('Day')
plt.legend()
plt.show()
