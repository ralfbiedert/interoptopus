import csv
import sys
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np
from matplotlib.colors import TwoSlopeNorm

root = Path(__file__).resolve().parent.parent.parent


def load_csv(path, wire_col="wire_ns", proto_col="proto_ns"):
    rows = []
    with open(path) as f:
        reader = csv.DictReader(f)
        for row in reader:
            rows.append({
                "N": int(row["N"]),
                "S": int(row["S"]),
                "wire": int(row[wire_col]),
                "proto": int(row[proto_col]),
            })
    return rows


def make_figure(rows, title, out_name):
    ns = sorted(set(r["N"] for r in rows))
    ss = sorted(set(r["S"] for r in rows))
    lookup = {(r["N"], r["S"]): r for r in rows}

    def ns_to_us(val):
        return val / 1000.0

    fig, axes = plt.subplots(1, 3, figsize=(16, 5))

    # --- Plot 1: Grouped bars at S=100 ---
    ax = axes[0]
    s_val = 100
    wire_vals = [ns_to_us(lookup[(n, s_val)]["wire"]) for n in ns]
    proto_vals = [ns_to_us(lookup[(n, s_val)]["proto"]) for n in ns]
    x = np.arange(len(ns))
    w = 0.35
    ax.bar(x - w / 2, wire_vals, w, label="Wire", color="#3b82f6")
    ax.bar(x + w / 2, proto_vals, w, label="Protobuf", color="#ef4444")
    ax.set_xticks(x)
    ax.set_xticklabels([f"{n}" for n in ns])
    ax.set_xlabel("N (elements)")
    ax.set_ylabel("\u00b5s per call")
    ax.set_title(f"Wire vs Protobuf (S={s_val})")
    ax.legend()
    ax.grid(axis="y", alpha=0.3)

    # --- Plot 2: Ratio heatmap ---
    ax = axes[1]
    ratio_matrix = np.array([
        [lookup[(n, s)]["proto"] / max(lookup[(n, s)]["wire"], 1) for s in ss]
        for n in ns
    ])
    norm = TwoSlopeNorm(vmin=0.5, vcenter=1.0, vmax=3.0)
    im = ax.imshow(ratio_matrix, cmap="RdYlGn", aspect="auto", norm=norm)
    ax.set_xticks(range(len(ss)))
    ax.set_xticklabels([f"S={s}" for s in ss], fontsize=9)
    ax.set_yticks(range(len(ns)))
    ax.set_yticklabels([f"{n}" for n in ns])
    ax.set_ylabel("N (elements)")
    ax.set_title("Protobuf / Wire ratio\n(>1 = Wire faster)")
    for i in range(len(ns)):
        for j in range(len(ss)):
            ax.text(j, i, f"{ratio_matrix[i, j]:.1f}x", ha="center", va="center", fontsize=10, fontweight="bold")
    fig.colorbar(im, ax=ax, shrink=0.8)

    # --- Plot 3: Line plot at S=1000 ---
    ax = axes[2]
    s_val = 1000
    wire_line = [ns_to_us(lookup[(n, s_val)]["wire"]) for n in ns]
    proto_line = [ns_to_us(lookup[(n, s_val)]["proto"]) for n in ns]
    ax.plot(ns, wire_line, "o-", label="Wire", color="#3b82f6", linewidth=2)
    ax.plot(ns, proto_line, "^-", label="Protobuf", color="#ef4444", linewidth=2)
    ax.set_xlabel("N (elements)")
    ax.set_ylabel("\u00b5s per call")
    ax.set_title(f"Scaling with N (S={s_val})")
    ax.legend()
    ax.grid(alpha=0.3)

    fig.suptitle(title, fontsize=13, fontweight="bold")
    fig.tight_layout(rect=[0, 0.06, 1, 0.95])
    fig.text(0.5, 0.01,
        "Note: Protobuf has a slight advantage in that it does not FFI-allocate, so it could not be used via async.",
        ha="center", fontsize=9, fontstyle="italic", color="#555555")

    out_path = root / out_name
    fig.savefig(out_path, dpi=150, bbox_inches="tight")
    print(f"Saved to {out_path}")


# Vec<String>
vec_csv = root / "wire_vs_protobuf_vec_string.csv"
if vec_csv.exists():
    rows = load_csv(vec_csv)
    make_figure(rows,
        "Wire vs Protobuf: Vec<String> \u2014 C# serialize \u2192 FFI \u2192 Rust deserialize",
        "wire_vs_protobuf_vec_string.png")

# HashMap<String, String>
map_csv = root / "wire_vs_protobuf_hashmap_string.csv"
if map_csv.exists():
    rows = load_csv(map_csv)
    make_figure(rows,
        "Wire vs Protobuf: HashMap<String, String> \u2014 C# serialize \u2192 FFI \u2192 Rust deserialize",
        "wire_vs_protobuf_hashmap_string.png")

plt.show()
