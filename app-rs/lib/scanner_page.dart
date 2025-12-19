import 'package:flutter/material.dart';
import 'package:flutter_blue_plus/flutter_blue_plus.dart';

class ScannerPage extends StatelessWidget {
  final BluetoothDevice? connectedDevice;
  final List<ScanResult> scanResults;
  final bool isScanning;
  final VoidCallback onScan;
  final Function(BluetoothDevice) onConnect;
  final Function(BluetoothDevice) onDisconnect;
  final VoidCallback onBack;

  const ScannerPage({
    super.key,
    required this.connectedDevice,
    required this.scanResults,
    required this.isScanning,
    required this.onScan,
    required this.onConnect,
    required this.onDisconnect,
    required this.onBack,
  });

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.all(24.0),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            children: [
              IconButton(
                icon: const Icon(Icons.arrow_back_ios),
                onPressed: onBack,
              ),
              const Text("Connectivity", style: TextStyle(fontSize: 28, fontWeight: FontWeight.bold)),
            ],
          ),
          const SizedBox(height: 20),
          
          if (connectedDevice != null)
            Card(
              color: Colors.blue.withOpacity(0.1),
              child: ListTile(
                leading: const Icon(Icons.bluetooth_connected, color: Colors.blue),
                title: Text(connectedDevice!.platformName.isNotEmpty ? connectedDevice!.platformName : "Connected Device"),
                subtitle: Text(connectedDevice!.remoteId.str),
                trailing: TextButton(
                  onPressed: () => onDisconnect(connectedDevice!),
                  child: const Text("DISCONNECT", style: TextStyle(color: Colors.red)),
                ),
              ),
            ),

          const SizedBox(height: 20),
          const Text("NEARBY SCOOTERS", style: TextStyle(fontSize: 12, fontWeight: FontWeight.bold, color: Colors.grey)),
          const Divider(),
          
          Expanded(
            child: scanResults.isEmpty && !isScanning
                ? const Center(child: Text("No devices found nearby", style: TextStyle(color: Colors.grey)))
                : ListView.builder(
                    itemCount: scanResults.length,
                    itemBuilder: (context, index) {
                      final result = scanResults[index];
                      final name = result.advertisementData.localName.isNotEmpty 
                          ? result.advertisementData.localName 
                          : result.device.platformName;
                      return ListTile(
                        leading: const Icon(Icons.bluetooth),
                        title: Text(name.isNotEmpty ? name : "Unknown"),
                        subtitle: Text("${result.device.remoteId.str} (${result.rssi} dBm)"),
                        onTap: () => onConnect(result.device),
                      );
                    },
                  ),
          ),
          
          SizedBox(
            width: double.infinity,
            child: ElevatedButton.icon(
              onPressed: isScanning ? null : onScan,
              icon: Icon(isScanning ? Icons.bluetooth_searching : Icons.search),
              label: Text(isScanning ? 'SCANNING...' : 'START SCAN'),
              style: ElevatedButton.styleFrom(
                padding: const EdgeInsets.all(16),
                backgroundColor: Colors.blue.withOpacity(0.2),
              ),
            ),
          ),
        ],
      ),
    );
  }
}
