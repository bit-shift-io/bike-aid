package com.bitshift.bike_aid;

import android.bluetooth.BluetoothAdapter;
import android.bluetooth.BluetoothDevice;
import android.bluetooth.BluetoothManager;
import android.bluetooth.le.BluetoothLeScanner;
import android.bluetooth.le.ScanCallback;
import android.bluetooth.le.ScanResult;
import android.content.Context;
import android.os.Handler;

import java.util.Set;

public class BLE {
    // https://developer.android.com/develop/connectivity/bluetooth/ble/find-ble-devices


    private BluetoothLeScanner bluetoothLeScanner;
    private BluetoothAdapter bluetoothAdapter;

    private Logger log;

    private boolean scanning;
    private Handler handler = new Handler();
    // Stops scanning after 10 seconds.
    private static final long SCAN_PERIOD = 10000;

    private String DEVICE_NAME = "BScooter";
    private Context context;


    // Device scan callback.
    private ScanCallback leScanCallback =
            new ScanCallback() {
                @Override
                public void onScanResult(int callbackType, ScanResult result) {
                    super.onScanResult(callbackType, result);
                    if (result.getDevice().getName() == null)
                            return;

                    //log.append("BLE Scan: " + result.getDevice().getName());
                    if (result.getDevice().getName().equals(DEVICE_NAME)) {
                        log.info("SCOOTER FOUND!");
                        scanning = false;
                        bluetoothLeScanner.stopScan(leScanCallback);
                    }
                }
            };



    BLE(Context c, Logger l) {
        context = c;
        log = l;

        BluetoothManager bluetoothManager = (BluetoothManager) context.getSystemService(Context.BLUETOOTH_SERVICE);
        bluetoothAdapter = bluetoothManager.getAdapter();

        bluetoothLeScanner = bluetoothAdapter.getBluetoothLeScanner();

        Set<BluetoothDevice> bondedDevices = bluetoothAdapter.getBondedDevices();

        if (bondedDevices != null)
            log.info("device bonded!");

        assert bondedDevices != null;
        for (BluetoothDevice dev : bondedDevices) {
            log.info(dev.getName());
        }



       // if (bluetoothAdapter == null || !bluetoothAdapter.isEnabled())
        startScan();
    }


    public boolean enableBluetooth() {
        if (bluetoothAdapter.isEnabled()) {
            return true;
        } else {
            String intentString = BluetoothAdapter.ACTION_REQUEST_ENABLE;

        }
        return true;
    }


    public void startScan() {
        if (!scanning) {
            // Stops scanning after a predefined scan period.
            handler.postDelayed(new Runnable() {
                @Override
                public void run() {
                    scanning = false;
                    bluetoothLeScanner.stopScan(leScanCallback);
                }
            }, SCAN_PERIOD);

            scanning = true;
            bluetoothLeScanner.startScan(leScanCallback);
        } else {
            scanning = false;
            bluetoothLeScanner.stopScan(leScanCallback);
        }

    }



}
