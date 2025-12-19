import 'dart:io';
import 'dart:async';
import 'dart:convert';
import 'package:flutter/material.dart';
import 'package:flutter_blue_plus/flutter_blue_plus.dart';
import 'package:permission_handler/permission_handler.dart';
import 'package:bike_aid/src/rust/api/protocol.dart';
import 'package:bike_aid/src/rust/frb_generated.dart';
import 'dashboard_page.dart';
import 'scanner_page.dart';

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
  static const String targetDeviceName = "BScooter";
  List<ScanResult> _scanResults = [];
  bool _scanning = false;
  bool _isConnecting = false;
  
  BluetoothDevice? _connectedDevice;
  ScooterState? _scooterState;
  StreamSubscription<BluetoothConnectionState>? _connectionSubscription;
  Timer? _reconnectTimer;

  final List<String> _logs = [];
  final ScrollController _logScrollController = ScrollController();
  final TextEditingController _uartController = TextEditingController();
  final PageController _pageController = PageController();

  @override
  void initState() {
    super.initState();
    _initBridgeAndState();
    
    // Start the persistent auto-reconnect loop
    _startReconnectLoop();

    // Listen to scan results
    FlutterBluePlus.scanResults.listen((results) {
      if (mounted) {
        setState(() {
          _scanResults = results;
        });

        // 🚀 Auto-connect logic (Java port)
        if (_connectedDevice == null && !_isConnecting) {
          for (var result in results) {
            final name = result.advertisementData.localName.isNotEmpty 
                ? result.advertisementData.localName 
                : result.device.platformName;
            
            if (name == targetDeviceName) {
              _connect(result.device);
              break;
            }
          }
        }
      }
    });

    FlutterBluePlus.isScanning.listen((state) {
      if (mounted) {
        setState(() {
          _scanning = state;
        });
      }
    });

    // Start scanning immediately (Java port)
    _scan();
  }

  Future<void> _initBridgeAndState() async {
    // Initialize Rust in background so UI isn't blocked on splash
    await RustLib.init();
    
    // Initial data
    final state = await ScooterState.default_();
    
    if (mounted) {
      setState(() {
        _scooterState = state;
      });
    }
  }

  Future<void> _checkBondedDevices() async {
    final bonded = await FlutterBluePlus.bondedDevices;
    for (var device in bonded) {
      final name = device.platformName;
      if (name == targetDeviceName && _connectedDevice == null && !_isConnecting) {
        _connect(device);
        break;
      }
    }
  }

  Future<void> _scan() async {
    // 1. Request Permissions (Mobile only)
    if (Platform.isAndroid || Platform.isIOS) {
      Map<Permission, PermissionStatus> statuses = await [
        Permission.bluetoothScan,
        Permission.bluetoothConnect,
        Permission.location,
      ].request();

      if (statuses.values.any((element) => element.isDenied)) {
        if (mounted) {
          ScaffoldMessenger.of(context).showSnackBar(
            const SnackBar(content: Text("Bluetooth & Location permissions are required.")),
          );
        }
        return;
      }
    }

    // 2. Check if Bluetooth is ON
    if (await FlutterBluePlus.adapterState.first != BluetoothAdapterState.on) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          const SnackBar(content: Text("Please turn on Bluetooth.")),
        );
      }
      return;
    }

    // 3. Start Scanning (Filtered for Scooter)
    try {
      _addLog("start scan");
      await FlutterBluePlus.startScan(
        timeout: const Duration(seconds: 15), 
        androidUsesFineLocation: true,
        withNames: [targetDeviceName],
      );
    } catch (e) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text("Scan Error: $e")),
        );
      }
    }
  }

  @override
  void dispose() {
    _connectionSubscription?.cancel();
    _reconnectTimer?.cancel();
    _logScrollController.dispose();
    _uartController.dispose();
    _pageController.dispose();
    super.dispose();
  }

  void _addLog(String message) {
    if (mounted) {
      setState(() {
        final time = DateTime.now();
        final timestamp = "${time.hour.toString().padLeft(2, '0')}:${time.minute.toString().padLeft(2, '0')}";
        _logs.add("$timestamp $message");
        if (_logs.length > 200) _logs.removeAt(0);
      });
      // Scroll to bottom
      WidgetsBinding.instance.addPostFrameCallback((_) {
        if (_logScrollController.hasClients) {
          _logScrollController.jumpTo(_logScrollController.position.maxScrollExtent);
        }
      });
    }
  }

  void _startReconnectLoop() {
    _reconnectTimer?.cancel();
    _reconnectTimer = Timer.periodic(const Duration(seconds: 5), (timer) async {
      if (_connectedDevice == null && !_isConnecting) {
        // Try bonded first
        await _checkBondedDevices();
        // If still not connecting, start a scan if not already scanning
        if (!_isConnecting && !_scanning) {
          _scan();
        }
      }
    });
  }

  Future<void> _connect(BluetoothDevice device) async {
    if (_isConnecting) return;
    
    try {
      setState(() => _isConnecting = true);
      
      if (_scanning) {
        _addLog("stop scan");
        await FlutterBluePlus.stopScan();
      }
      
      _addLog("connecting: ${device.platformName}");
      await device.connect(autoConnect: false, timeout: const Duration(seconds: 10));
      
      await _connectionSubscription?.cancel();
      _connectionSubscription = device.connectionState.listen((state) {
        if (state == BluetoothConnectionState.disconnected) {
          if (mounted) {
            _addLog("disconnected from device");
            ScaffoldMessenger.of(context).showSnackBar(
              SnackBar(
                content: Text("Scooter Disconnected: ${device.platformName}"),
                backgroundColor: Colors.red.withOpacity(0.8),
                duration: const Duration(seconds: 2),
              ),
            );
            setState(() {
              _connectedDevice = null;
              _isConnecting = false;
            });
            // Loop will handle it
          }
        } else if (state == BluetoothConnectionState.connected) {
          if (mounted) {
             _addLog("connected: ${device.platformName}");
             ScaffoldMessenger.of(context).showSnackBar(
              SnackBar(
                content: Text("Scooter Connected: ${device.platformName}"),
                backgroundColor: Colors.green.withOpacity(0.8),
                duration: const Duration(seconds: 2),
              ),
            );
          }
        }
      });

      setState(() {
        _connectedDevice = device;
        _isConnecting = false;
      });

      // Discover Services
      List<BluetoothService> services = await device.discoverServices();
      for (var service in services) {
        for (var characteristic in service.characteristics) {
          if (characteristic.properties.notify || characteristic.properties.indicate) {
            await characteristic.setNotifyValue(true);
              characteristic.onValueReceived.listen((value) {
                if (mounted && _scooterState != null) {
                  // Get 16-bit UUID string from 128-bit
                  final fullUuid = characteristic.uuid.toString().toUpperCase();
                  String uuidStr = "";
                  if (fullUuid.startsWith("0000") && fullUuid.endsWith("0000-1000-8000-00805F9B34FB")) {
                    uuidStr = fullUuid.substring(4, 8);
                  } else {
                    uuidStr = fullUuid;
                  }

                  if (uuidStr == "0003") {
                    _addLog(utf8.decode(value));
                  }

                  final result = parseCharacteristicData(
                    state: _scooterState!,
                    uuid16: uuidStr,
                    data: value,
                  );

                  if (result.log != null) {
                    _addLog(result.log!);
                  }

                  setState(() {
                    _scooterState = result.state;
                  });
                }
              });
          }
          // Also read current value if possible
          if (characteristic.properties.read) {
            final value = await characteristic.read();
            if (mounted && _scooterState != null) {
              final uuidStr = characteristic.uuid.toString().substring(4, 8);
              final result = parseCharacteristicData(
                state: _scooterState!,
                uuid16: uuidStr,
                data: value,
              );
              setState(() {
                _scooterState = result.state;
              });
            }
          }
        }
      }
    } catch (e) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text("Connection error: $e")));
      }
    }
  }

  Future<void> _sendCommand(ScooterCommand command) async {
    if (_connectedDevice == null || _scooterState == null) return;
    
    final action = getCommandAction(command: command, currentState: _scooterState!);
    
    // Log command
    _addLog("> sending command: ${command.toString().split('_').last}");
    
    try {
      final services = await _connectedDevice!.discoverServices();
      final service = services.firstWhere((s) => s.uuid == Guid(action.serviceUuid));
      final char = service.characteristics.firstWhere((c) => c.uuid == Guid(action.characteristicUuid));
      await char.write(action.bytes);
    } catch (e) {
      debugPrint("Write failed: $e");
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(
            content: Text("Command Failed: $e"),
            backgroundColor: Colors.orange.withOpacity(0.8),
          ),
        );
      }
    }
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
              scooterState: _scooterState,
              isConnecting: _isConnecting,
              isScanning: _scanning,
              logs: _logs,
              logScrollController: _logScrollController,
              uartController: _uartController,
              onSendCommand: _sendCommand,
              onClearLogs: () => setState(() => _logs.clear()),
            ),
            ScannerPage(
              connectedDevice: _connectedDevice,
              scanResults: _scanResults,
              isScanning: _scanning,
              onScan: _scan,
              onConnect: (device) {
                _connect(device);
                _pageController.animateToPage(0, duration: const Duration(milliseconds: 300), curve: Curves.easeInOut);
              },
              onDisconnect: (device) async {
                await device.disconnect();
                setState(() => _connectedDevice = null);
              },
              onBack: () => _pageController.animateToPage(0, duration: const Duration(milliseconds: 300), curve: Curves.easeInOut),
            ),
          ],
        ),
      ),
    );
  }
}
