package com.bitshift.bike_aid;

import android.bluetooth.BluetoothAdapter;
import android.bluetooth.BluetoothDevice;
import android.bluetooth.BluetoothGatt;
import android.bluetooth.BluetoothGattCallback;
import android.bluetooth.BluetoothGattCharacteristic;
import android.bluetooth.BluetoothGattDescriptor;
import android.bluetooth.BluetoothGattService;
import android.bluetooth.BluetoothManager;
import android.bluetooth.le.BluetoothLeScanner;
import android.bluetooth.le.ScanCallback;
import android.bluetooth.le.ScanResult;
import android.content.Context;
import android.os.Handler;

import java.util.List;
import java.util.Set;
import java.util.UUID;

public class BLE {
    // ==== notes ====
    /*
    This class handles the ble adaptor and connection to the device
    Once we are connected, we hand off to the gatt class to handle read and write of data
     */

    // if we have ble issues
    // UIThread (with a handler, local service, or Activity#runOnUiThread). Follow this rule of thumb and you will hopefully avoid this dreadful problem.

    // ==== variables ====
    private static BLE mInstance = new BLE();
    private static final Logger log = Logger.getInstance();
    private BluetoothLeScanner bluetoothLeScanner;
    private BluetoothAdapter bluetoothAdapter;
    private boolean scanning;
    private Handler handler = new Handler();
    private static final long SCAN_PERIOD = 60000; // 60 seconds
    private String DEVICE_NAME = "BScooter";
    BluetoothDevice mDevice;
    boolean deviceFound = false;
    private Context context;


    // ==== callbacks ====
    private ScanCallback scanCallback =
            new ScanCallback() {
                @Override
                public void onScanResult(int callbackType, ScanResult result) {
                    super.onScanResult(callbackType, result);
                    BluetoothDevice dev = result.getDevice();
                    if (dev == null || dev.getName() == null)
                        return;

                    log.info("BLE Scan: " + dev.getName());
                    if (isWantedDevice(dev)) {
                        connectDevice();
                        stopScan();
                    }
                }
            };


    // ==== functions ====

    private BLE() {}


    public static BLE getInstance() {
        return mInstance;
    }


    public void init(Context c) {
        context = c;
        //BluetoothManager bluetoothManager = (BluetoothManager) context.getSystemService(Context.BLUETOOTH_SERVICE);
        //bluetoothAdapter = bluetoothManager.getAdapter();
        bluetoothAdapter = BluetoothAdapter.getDefaultAdapter();
        enableBluetooth();

        bluetoothLeScanner = bluetoothAdapter.getBluetoothLeScanner();
        if (bluetoothLeScanner == null) {
            log.info("BLE: no bluetooth adapter available, please enable bluetooth");
            return;
        }

        // check for bonded devices
        Set<BluetoothDevice> bondedDevices = bluetoothAdapter.getBondedDevices();
        if (bondedDevices != null) {
            for (BluetoothDevice dev : bondedDevices) {
                if (isWantedDevice(dev)) {
                    connectDevice();
                    break;
                }
            }
        }

        // device not found, scan
        if (!deviceFound)
            startScan();
    }


    public void connectDevice() {
        mDevice.createBond();
        // device connected will register the callbacks here
        // and so i have the gatt class to handle it

        mDevice.connectGatt(context, true, new GATT());
    }


    // find if this is the wanted device, by name
    public boolean isWantedDevice(BluetoothDevice dev) {
        if (!dev.getName().equals(DEVICE_NAME))
            return false;

        mDevice = dev;
        deviceFound = true;
        log.info("BLE wanted device found");
        return true;
    }


    public boolean enableBluetooth() {
        if (bluetoothAdapter.isEnabled()) {
            return true;
        } else {
            log.info("BLE: enable adapter");
            bluetoothAdapter.enable();
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
                    log.info("BLE stop scan");
                    scanning = false;
                    bluetoothLeScanner.stopScan(scanCallback);
                }
            }, SCAN_PERIOD);

            log.info("BLE start scan");
            scanning = true;
            bluetoothLeScanner.startScan(scanCallback);
        } else {
            stopScan();
        }

    }

    public void stopScan() {
        scanning = false;
        bluetoothLeScanner.stopScan(scanCallback);
    }

}
