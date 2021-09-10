
import numpy as np
import pandas as pd
import json
import csv
from mpl_toolkits import mplot3d
import matplotlib.pyplot as plt


def unpack():
    print("unpacking formflight-rs tracking scenario results")

    x = 0
    y = 0
    z = 0

    return (x, y, z)

def plot_trajectory(data):
    print("plotting trajectory in world frame")

    with open('../../../../../test.csv', newline='') as csvfile:
        spamreader = csv.reader(csvfile, delimiter=' ', quotechar='|')
        for row in spamreader:
            print(', '.join(row))

    fig = plt.figure()
    ax = plt.axes(projection='3d')

    # Data for a three-dimensional line
    zline = np.linspace(0, 15, 1000)
    xline = np.sin(zline)
    yline = np.cos(zline)
    ax.plot3D(xline, yline, zline, 'gray')

    # Data for three-dimensional scattered points
    zdata = 15 * np.random.random(100)
    xdata = np.sin(zdata) + 0.1 * np.random.randn(100)
    ydata = np.cos(zdata) + 0.1 * np.random.randn(100)
    ax.scatter3D(xdata, ydata, zdata, c=zdata, cmap='Greens');

    plt.show()



def main():

    data = unpack()
    plot_trajectory(data)
    # plot_assignments(data)

if __name__ == "__main__":
    main()

