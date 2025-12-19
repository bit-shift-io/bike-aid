import 'dart:async';
import 'package:flutter/material.dart';
import 'package:flutter_blue_plus/flutter_blue_plus.dart';
import 'package:bike_aid/src/rust/api/protocol.dart';
import 'logger_service.dart';
import 'dashboard_page.dart';
import 'scanner_page.dart';
import 'bluetooth_service.dart';

void main() {
  WidgetsFlutterBinding.ensureInitialized();
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Bike Aid',
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(
          seedColor: Colors.blue,
          brightness: Brightness.dark,
        ),
        useMaterial3: true,
      ),
      themeMode: ThemeMode.dark,
      home: const MyHomePage(title: 'Bike Aid - Scooter Console'),
    );
  }
}

class MyHomePage extends StatefulWidget {
  const MyHomePage({super.key, required this.title});
  final String title;

  @override
  State<MyHomePage> createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage> {
  final ScooterBluetoothService _bluetoothService = ScooterBluetoothService();
  final LoggerService _loggerService = LoggerService();
  
  final ScrollController _logScrollController = ScrollController();
  final TextEditingController _uartController = TextEditingController();
  final PageController _pageController = PageController();
  
  StreamSubscription? _logSubscription;

  @override
  void initState() {
    super.initState();
    
    // Initialize Service
    _bluetoothService.init();
    
    // Listen for state changes to rebuild UI
    _bluetoothService.addListener(_onServiceUpdate);
    _loggerService.addListener(_onServiceUpdate);

    // Pipe Bluetooth logs to Logger
    _logSubscription = _bluetoothService.logStream.listen((message) {
      _loggerService.add(message);
      // Auto-scroll
      if (mounted) {
        WidgetsBinding.instance.addPostFrameCallback((_) {
          if (_logScrollController.hasClients) {
            _logScrollController.jumpTo(_logScrollController.position.maxScrollExtent);
          }
        });
      }
    });
  }

  void _onServiceUpdate() {
    if (mounted) {
      setState(() {});
    }
  }

  @override
  void dispose() {
    _bluetoothService.removeListener(_onServiceUpdate);
    _loggerService.removeListener(_onServiceUpdate);
    
    _bluetoothService.dispose();
    _loggerService.dispose();
    
    _logSubscription?.cancel();
    _logScrollController.dispose();
    _uartController.dispose();
    _pageController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: Colors.black,
      body: SafeArea(
        child: PageView(
          controller: _pageController,
          children: [
            DashboardPage(
              scooterState: _bluetoothService.scooterState,
              isConnecting: _bluetoothService.isConnecting,
              isScanning: _bluetoothService.isScanning,
              logs: _loggerService.logs,
              logScrollController: _logScrollController,
              uartController: _uartController,
              onSendCommand: _bluetoothService.sendCommand,
              onClearLogs: _loggerService.clear,
            ),
            ScannerPage(
              connectedDevice: _bluetoothService.connectedDevice,
              scanResults: _bluetoothService.scanResults,
              isScanning: _bluetoothService.isScanning,
              onScan: _bluetoothService.scan,
              onConnect: (device) {
                _bluetoothService.connect(device);
                _pageController.animateToPage(0, duration: const Duration(milliseconds: 300), curve: Curves.easeInOut);
              },
              onDisconnect: (device) async {
                await device.disconnect();
                // Service will handle state update via listener
              },
              onBack: () => _pageController.animateToPage(0, duration: const Duration(milliseconds: 300), curve: Curves.easeInOut),
            ),
          ],
        ),
      ),
    );
  }
}
