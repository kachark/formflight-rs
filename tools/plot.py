
import sys, getopt
import pandas as pd
import matplotlib.pyplot as plt
import matplotlib.animation as animation
from mpl_toolkits import mplot3d

# 3D histogram animation update function
def animate(num, data, hist):

    # update all the plot objects
    for (name, plot_obj) in hist.items():
        # only update with current time step (not the previous iteration history)
        x = data[name].iloc[num]
        y = data[name + ".1"].iloc[num]
        z = data[name + ".2"].iloc[num]
        plot_obj.set_data(x, y)
        plot_obj.set_3d_properties(z)

    # return tuple of modified plot objects
    return tuple(hist.values())


# Generate plot objects for each agent and target trajectory
def get_agent_target_plots(ax, trajectories_df):

    # "Entity#.x" -> #: number, x: state component
    # Filter all agents trajeectories and get the column names without mangling or dots
    agent_names = [x for x in trajectories_df.filter(like='Agent').columns.to_list() if "." not in x]
    target_names = [x for x in trajectories_df.filter(like='Target').columns.to_list() if "." not in x]

    # Generate plotting objects
    target_hist = {}
    agent_hist = {}

    # plot stationary targets
    for name in target_names:
        x = trajectories_df[name]
        y = trajectories_df[name + ".1"]
        z = trajectories_df[name + ".2"]

        # plot initial state
        ax.plot(x.iloc[0], y.iloc[0], z.iloc[0], c='r', marker='x')

        # plot time history
        target_hist[name] = ax.plot(x, y, z, c='r', marker='x')[0]

    for name in agent_names:
        x = trajectories_df[name]
        y = trajectories_df[name + ".1"]
        z = trajectories_df[name + ".2"]

        # plot initial state
        # plt.plot(x.iloc[0], y.iloc[0], z.iloc[0], c='k', marker='o')

        # plot time history
        agent_hist[name] = ax.plot(x, y, z, c='b', marker='o')[0]

    return agent_hist, target_hist


def main(argv):

    saveflag = False
    try:
        opts, args = getopt.getopt(argv, "s")
    except getopt.GetoptError:
        print("plot.py -s")
        sys.exit(2)

    for opt, arg in opts:
        if opt == '-s':
            saveflag = True
        else:
            saveflag = False

    print("reading data...")
    trajectories_df = pd.read_csv("../results.csv")
    num_data_points = len(trajectories_df.index)

    print("plotting...")
    fig = plt.figure()
    ax = plt.axes(projection='3d')
    ax.set_xlabel("x")
    ax.set_ylabel("y")
    ax.set_zlabel("z")

    # Generate 3D plotting objects as they relate to individual Agents/Targets
    agent_hist, target_hist = get_agent_target_plots(ax, trajectories_df)

    # Animate 3D trajectories
    ani = animation.FuncAnimation(fig, animate, frames=num_data_points, fargs=(trajectories_df, agent_hist), interval=50, blit=True)

    if saveflag:
        print("saving...this may take a while")
        ani.save(r'trajectory_animation.gif', writer='imagemagick', fps=30)

    plt.show()

if __name__ == "__main__":
    main(sys.argv[1:])

