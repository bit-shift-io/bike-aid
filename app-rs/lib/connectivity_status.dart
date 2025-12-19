import 'package:flutter/material.dart';
import 'package:flutter_blue_plus/flutter_blue_plus.dart';
import 'package:bike_aid/i18n/strings.g.dart';
import 'ble_scanner_list.dart';

class ConnectivityStatus extends StatelessWidget {
  final BluetoothDevice? connectedDevice;
  final List<ScanResult> scanResults;
  final bool isConnecting;
  final bool isScanning;
  final VoidCallback onScan;
  final Function(BluetoothDevice) onConnect;
  final Function(BluetoothDevice) onDisconnect;
  final VoidCallback onGoToScanner;

  const ConnectivityStatus({
    super.key,
    required this.connectedDevice,
    required this.scanResults,
    required this.isConnecting,
    required this.isScanning,
    required this.onScan,
    required this.onConnect,
    required this.onDisconnect,
    required this.onGoToScanner,
  });

  @override
  Widget build(BuildContext context) {
    String statusText = "";
    Color statusColor = Colors.grey;
    IconData statusIcon = Icons.bluetooth_disabled;

    if (connectedDevice != null) {
      statusText = connectedDevice!.platformName.isNotEmpty
          ? connectedDevice!.platformName
          : t.log_page.unknown;
      statusColor = Colors.blue;
      statusIcon = Icons.bluetooth_connected;
    } else if (isConnecting) {
      statusText = "CONNECTING...";
      statusColor = Colors.orange;
      statusIcon = Icons.bluetooth_searching;
    } else if (isScanning) {
      statusText = t.log_page.scanning.toUpperCase();
      statusColor = Colors.green;
      statusIcon = Icons.bluetooth_searching;
    } else {
      statusText = "DISCONNECTED";
      statusColor = Colors.red;
      statusIcon = Icons.bluetooth_disabled;
    }

    return Column(
      mainAxisSize: MainAxisSize.min,
      children: [
        InkWell(
          onTap: onGoToScanner,
          child: Container(
            padding: const EdgeInsets.symmetric(horizontal: 24, vertical: 16),
            color: Colors.black,
            child: Row(
              children: [
                Icon(statusIcon, color: statusColor, size: 32),
                const SizedBox(width: 16),
                Expanded(
                  child: Column(
                    mainAxisAlignment: MainAxisAlignment.center,
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Text(
                        statusText,
                        style: TextStyle(
                          color: statusColor,
                          fontSize: 20,
                          fontWeight: FontWeight.bold,
                        ),
                      ),
                      if (connectedDevice != null)
                        Text(
                          connectedDevice!.remoteId.str,
                          style: const TextStyle(
                            color: Colors.white54,
                            fontSize: 12,
                          ),
                        ),
                    ],
                  ),
                ),
                const Icon(
                  Icons.chevron_right,
                  color: Colors.white24,
                  size: 32,
                ),
              ],
            ),
          ),
        ),
        const Divider(height: 1, color: Colors.white24),
        // Scanner List integrated here
        Flexible(
          child: BleScannerList(
            connectedDevice: connectedDevice,
            scanResults: scanResults,
            isScanning: isScanning,
            onScan: onScan,
            onConnect: onConnect,
            onDisconnect: onDisconnect,
          ),
        ),
      ],
    );
  }
}
