import 'package:flutter/material.dart';
import 'package:flutter_blue_plus/flutter_blue_plus.dart';
import 'package:bike_aid/i18n/strings.g.dart';

class BleScannerList extends StatelessWidget {
  final BluetoothDevice? connectedDevice;
  final List<ScanResult> scanResults;
  final bool isScanning;
  final VoidCallback onScan;
  final Function(BluetoothDevice) onConnect;
  final Function(BluetoothDevice) onDisconnect;

  const BleScannerList({
    super.key,
    required this.connectedDevice,
    required this.scanResults,
    required this.isScanning,
    required this.onScan,
    required this.onConnect,
    required this.onDisconnect,
  });

  @override
  Widget build(BuildContext context) {
    return Column(
      mainAxisSize: MainAxisSize.min,
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Padding(
          padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
          child: Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              Text(
                t.log_page.nearby,
                style: const TextStyle(
                  fontSize: 12,
                  fontWeight: FontWeight.bold,
                  color: Colors.grey,
                ),
              ),
              IconButton(
                icon: Icon(
                  isScanning ? Icons.stop : Icons.search,
                  color: Colors.blue,
                  size: 20,
                ),
                onPressed: onScan,
                padding: EdgeInsets.zero,
                constraints: const BoxConstraints(),
              ),
            ],
          ),
        ),
        const Divider(height: 1, color: Colors.white24),
        Flexible(
          child: ListView(
            shrinkWrap: true,
            padding: const EdgeInsets.symmetric(horizontal: 8),
            children: [
              if (connectedDevice != null)
                Card(
                  color: Colors.blue.withOpacity(0.1),
                  child: ListTile(
                    dense: true,
                    leading: const Icon(
                      Icons.bluetooth_connected,
                      color: Colors.blue,
                    ),
                    title: Text(
                      connectedDevice!.platformName.isNotEmpty
                          ? connectedDevice!.platformName
                          : t.log_page.unknown,
                    ),
                    trailing: TextButton(
                      onPressed: () => onDisconnect(connectedDevice!),
                      child: Text(
                        t.log_page.disconnect,
                        style: const TextStyle(color: Colors.red),
                      ),
                    ),
                  ),
                ),

              ...scanResults.map((result) {
                final name = result.advertisementData.localName.isNotEmpty
                    ? result.advertisementData.localName
                    : result.device.platformName;
                final isConnected = result.device == connectedDevice;
                if (isConnected) return const SizedBox.shrink();

                return ListTile(
                  dense: true,
                  leading: const Icon(Icons.bluetooth),
                  title: Text(name.isNotEmpty ? name : t.log_page.unknown),
                  subtitle: Text(result.device.remoteId.str),
                  onTap: () => onConnect(result.device),
                );
              }),
              if (scanResults.isEmpty && !isScanning && connectedDevice == null)
                const Padding(
                  padding: EdgeInsets.all(16.0),
                  child: Center(
                    child: Text(
                      "No devices found",
                      style: TextStyle(color: Colors.grey, fontSize: 12),
                    ),
                  ),
                ),
            ],
          ),
        ),
      ],
    );
  }
}
