import matplotlib.pyplot as plt
import pandas as pd
from matplotlib.animation import FuncAnimation

# Setup side-by-side plots
fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(12, 5))

def animate(i):
    # --- Graph 1: Route (X, Y) ---
    try:
        df_points = pd.read_csv('points.csv', names=['X', 'Y'])
        ax1.clear()
        ax1.plot(df_points['X'], df_points['Y'], marker='o', color='blue', linestyle='-')
        ax1.set_title('Route (X, Y)')
        ax1.set_xlabel('X')
        ax1.set_ylabel('Y')
        ax1.grid(True, alpha=0.3)
    except Exception:
        pass  # Handle empty file or Rust write collision

    # --- Graph 2: Distance Over Time ---
    try:
        df_dist = pd.read_csv('distance.csv', names=['Distance'])
        ax2.clear()
        ax2.plot(df_dist.index, df_dist['Distance'], color='red', linestyle='-')
        ax2.set_title('Distance over Time')
        ax2.set_xlabel('Time Step')
        ax2.set_ylabel('Distance')
        ax2.grid(True, alpha=0.3)
    except Exception:
        pass  # Handle empty file or Rust write collision

    plt.tight_layout()

# Refresh every 500ms
ani = FuncAnimation(fig, animate, interval=500, cache_frame_data=False)
plt.show()
