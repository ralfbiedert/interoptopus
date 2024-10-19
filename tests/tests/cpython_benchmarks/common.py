DLL = "../../../target/release/interoptopus_reference_project.dll"
N = 1_000_000

import time

def bench(file, name, f, reference = 0):
    start = time.perf_counter()

    for i in range(N):
        f()

    end = time.perf_counter()
    nanos = (end-start) * 1000 * 1000 * 1000
    nanos_per_single = (nanos / N) - reference

    name = f"`{name}`"

    if reference > 0:
        print("|", name.ljust(50), "| {:,.0f} |".format(nanos_per_single), file=file)

    return nanos_per_single
