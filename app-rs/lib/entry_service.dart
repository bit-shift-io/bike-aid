import 'dart:async';
import 'package:connectivity_plus/connectivity_plus.dart';
import 'package:http/http.dart' as http;
import 'package:flutter/foundation.dart';

class EntryService {
  static final EntryService _instance = EntryService._internal();
  factory EntryService() => _instance;
  EntryService._internal();

  bool _isPending = false;
  StreamSubscription<List<ConnectivityResult>>? _subscription;

  Future<void> triggerEntry() async {
    if (_isPending) {
      debugPrint("Entry request already pending...");
      return;
    }

    final connectivityResult = await Connectivity().checkConnectivity();
    if (connectivityResult.contains(ConnectivityResult.wifi)) {
      _sendRequest();
    } else {
      debugPrint("Wifi not connected. Waiting for connection...");
      _isPending = true;
      _subscription = Connectivity().onConnectivityChanged.listen((results) {
        if (results.contains(ConnectivityResult.wifi)) {
          debugPrint("Wifi connected! Sending entry request...");
          _sendRequest();
          _cancelSubscription();
        }
      });
    }
  }

  Future<void> _sendRequest() async {
    try {
      final url = Uri.parse("http://iot.lan:1880/entry");
      final response = await http.get(url);
      debugPrint("Entry request sent. Status: ${response.statusCode}");
    } catch (e) {
      debugPrint("Error sending entry request: $e");
    } finally {
      _isPending = false;
    }
  }

  void _cancelSubscription() {
    _subscription?.cancel();
    _subscription = null;
  }
}
