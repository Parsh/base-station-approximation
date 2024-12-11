# Approximating Positions of Mobile Base Stations

## Problem Breakdown

### Objective
The goal is to approximate the positions of base stations (BS) based on smartphone data, including RSSI values, positions, timestamps, and cell IDs (CIs). The algorithm must handle RSSI values from different smartphone chipsets with varying ranges (e.g., [0-61] or [0-251]) and incorporate unassigned points to rule out impossible BS locations.

### Conversion Formula
The RSSI values can be converted to distances using the formula:
`| (rssi - maxrssi) * (radius / (maxrssi-1)) |`

### Assumptions
- The base station transmission radius (r) is fixed at 600 meters in an urban GSM scenario.
- The base station signal propagation is omni-directional.
- Points not assigned to a BS are considered outside the radii of all known BSs.
- RSSI values are chipset-specific and need normalization.

## Pseudocode

```
Algorithm ApproximateBaseStationLocations:

Input: SmartphoneData, MaxRSSI, r
Output: EstimatedBSPositions

Step 1: Initialize Variables
    BSPositions = {}          
    PointAssignments = {}  

Step 2: Process Data
    For each data entry in SmartphoneData:
        Normalize RSSI to [0, MaxRSSI] based on the device's range
        Calculate distance d = |(RSSI - MaxRSSI) * (r / (MaxRSSI - 1))|
        If d <= r:
            Assign point (x, y) to CI
        Else:
            Mark point as unassigned

Step 3: Handle Unassigned Points
    For each unassigned point:
        Exclude the point's location from potential regions of all known BS

Step 4: Estimate Base Station Positions
    For each CI in data:
        Collect all points assigned to CI
        Calculate the multilaterate(using circle intersection) as the initial BS position

Step 5: Output Results
    Return BSPositions

End Algorithm
```

## Consideration of the Time of Measurement

The time at which the RSSI measurements are recorded can significantly impact the accuracy and reliability of the base station position approximation, especially if the clocks of the individual smartphones are not synchronized.

### Impact of Unsynchronized Clocks
When the smartphone clocks are not synchronized, the RSSI measurements associated with each data point will have temporal offsets. This can introduce several challenges:

1. **Inconsistent Distance Estimates**: The distance calculations derived from the RSSI values will be based on measurements taken at different times. This can lead to inconsistencies, as the actual distance between the smartphone and base station may have changed between measurements.

2. **Temporal Misalignment**: The multilateration algorithm assumes that all the RSSI measurements correspond to the same point in time. When the measurements are not temporally aligned, the estimated base station position may not accurately represent the true location at any given moment.

3. **Erroneous Grouping by Cell ID**: If the cell ID (CI) assignments are not stable over time, the grouping of smartphone data points by CI may become unreliable. This can result in the multilateration algorithm using data points that do not actually correspond to the same base station.

### Potential Mitigation Strategies
To address the challenges posed by unsynchronized clocks, the following strategies could be considered:

1. **Clock Synchronization**: Implement a mechanism to synchronize the clocks of the participating smartphones, either through network-based time synchronization protocols or by incorporating GPS time information.

2. **Temporal Filtering**: Apply temporal filters to the input data, grouping RSSI measurements that fall within a specific time window. This can help mitigate the effects of clock offsets and ensure that the multilateration is performed on temporally aligned data points.

3. **Time-Aware Multilateration**: Modify the multilateration algorithm to explicitly consider the time stamps of the RSSI measurements. This may involve techniques like Kalman filtering or other state-space models that can incorporate the temporal dimension and provide more robust position estimates.

By addressing the impact of unsynchronized clocks, the overall reliability and accuracy of the base station position approximation can be improved, reducing the risk of false or inconsistent results.