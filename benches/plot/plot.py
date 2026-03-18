import csv
import sys
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np
from matplotlib.colors import TwoSlopeNorm

csv_path = "wire_vs_protobuf.csv"
if len(sys.argv) > 1:
    csv_path = Path(sys.argv[1])

rows = []
with open(csv_path) as f:
    reader = csv.DictReader(f)
    for row in reader:
        rows.append({
            "N": int(row["N"]),
            "S": int(row["S"]),
            "wire": int(row["wire_ns"]),
            "proto_alloc": int(row["proto_alloc_ns"]),
            "proto_prealloc": int(row["proto_prealloc_ns"]),
        })

ns = sorted(set(r["N"] for r in rows))
ss = sorted(set(r["S"] for r in rows))

lookup = {(r["N"], r["S"]): r for r in rows}

def ns_to_us(val):
    return val / 1000.0

fig, axes = plt.subplots(1, 3, figsize=(16, 5))

# --- Plot 1: Grouped bars, one group per N, comparing wire vs proto_prealloc at S=100 ---
ax = axes[0]
s_val = 100
wire_vals = [ns_to_us(lookup[(n, s_val)]["wire"]) for n in ns]
proto_vals = [ns_to_us(lookup[(n, s_val)]["proto_prealloc"]) for n in ns]
x = np.arange(len(ns))
w = 0.35
ax.bar(x - w / 2, wire_vals, w, label="Wire", color="#3b82f6")
ax.bar(x + w / 2, proto_vals, w, label="Protobuf (prealloc)", color="#ef4444")
ax.set_xticks(x)
ax.set_xticklabels([f"{n**3} obj" for n in ns])
ax.set_ylabel("\u00b5s per call")
ax.set_title(f"Wire vs Protobuf (S={s_val})")
ax.legend()
ax.grid(axis="y", alpha=0.3)

# --- Plot 2: Ratio (proto_prealloc / wire) heatmap ---
ax = axes[1]
ratio_matrix = np.array([
    [lookup[(n, s)]["proto_prealloc"] / lookup[(n, s)]["wire"] for s in ss]
    for n in ns
])
norm = TwoSlopeNorm(vmin=0.5, vcenter=1.0, vmax=3.0)
im = ax.imshow(ratio_matrix, cmap="RdYlGn", aspect="auto", norm=norm)
ax.set_xticks(range(len(ss)))
ax.set_xticklabels([f"S={s}" for s in ss], fontsize=9)
ax.set_yticks(range(len(ns)))
ax.set_yticklabels([f"{n**3} obj" for n in ns])
ax.set_title("Protobuf / Wire ratio\n(>1 = Wire faster)")
for i in range(len(ns)):
    for j in range(len(ss)):
        ax.text(j, i, f"{ratio_matrix[i, j]:.1f}x", ha="center", va="center", fontsize=10, fontweight="bold")
fig.colorbar(im, ax=ax, shrink=0.8)

# --- Plot 3: Line plot, us vs N for each method, at S=1000 ---
ax = axes[2]
s_val = 1000
wire_line = [ns_to_us(lookup[(n, s_val)]["wire"]) for n in ns]
proto_alloc_line = [ns_to_us(lookup[(n, s_val)]["proto_alloc"]) for n in ns]
proto_prealloc_line = [ns_to_us(lookup[(n, s_val)]["proto_prealloc"]) for n in ns]
ax.plot([n**3 for n in ns], wire_line, "o-", label="Wire", color="#3b82f6", linewidth=2)
ax.plot([n**3 for n in ns], proto_alloc_line, "s--", label="Protobuf (alloc)", color="#f59e0b", linewidth=2)
ax.plot([n**3 for n in ns], proto_prealloc_line, "^-", label="Protobuf (prealloc)", color="#ef4444", linewidth=2)
ax.set_xlabel("Total objects (N\u00b3)")
ax.set_ylabel("\u00b5s per call")
ax.set_title(f"Scaling with nesting depth (S={s_val})")
ax.legend()
ax.grid(alpha=0.3)

fig.suptitle("Wire vs Protobuf: C# serialize \u2192 FFI \u2192 Rust deserialize", fontsize=13, fontweight="bold")
fig.tight_layout(rect=[0, 0.06, 1, 0.95])
fig.text(0.5, 0.01,
    "Note: Protobuf has a slight advantage in that it does not FFI-allocate, so it could not be used via async.",
    ha="center", fontsize=9, fontstyle="italic", color="#555555")

out_path = "wire_vs_protobuf.png"
fig.savefig(out_path, dpi=150, bbox_inches="tight")
print(f"Saved to {out_path}")
plt.show()
