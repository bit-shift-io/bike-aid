import 'package:flutter/material.dart';
import 'package:bike_aid/src/rust/api/protocol.dart';
import 'dashboard.dart';
import 'log.dart';

class DashboardPage extends StatelessWidget {
  final ScooterState? scooterState;
  final bool isConnecting;
  final bool isScanning;
  final List<String> logs;
  final ScrollController logScrollController;
  final TextEditingController uartController;
  final Function(ScooterCommand) onSendCommand;
  final VoidCallback onClearLogs;

  const DashboardPage({
    super.key,
    required this.scooterState,
    required this.isConnecting,
    required this.isScanning,
    required this.logs,
    required this.logScrollController,
    required this.uartController,
    required this.onSendCommand,
    required this.onClearLogs,
  });

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        // 1. The 5-Row Dashboard (Top Section)
        ScooterDashboard(
          state: scooterState,
          isConnecting: isConnecting,
          isScanning: isScanning,
          onSendCommand: onSendCommand,
        ),

        const Divider(height: 2, color: Colors.white54, thickness: 2),

        // 2. Log Terminal & UART (Bottom Section)
        Expanded(
          child: LogTerminal(
            logs: logs,
            scrollController: logScrollController,
            uartController: uartController,
            onSendCommand: onSendCommand,
            onClearLogs: onClearLogs,
          ),
        ),
      ],
    );
  }
}
