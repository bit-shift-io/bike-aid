package com.bitshift.bike_aid;

import android.bluetooth.BluetoothAdapter;
import android.bluetooth.BluetoothDevice;
import android.bluetooth.BluetoothGatt;
import android.bluetooth.BluetoothGattCallback;
import android.bluetooth.BluetoothGattCharacteristic;
import android.bluetooth.BluetoothGattService;
import android.bluetooth.BluetoothManager;
import android.bluetooth.le.BluetoothLeScanner;
import android.bluetooth.le.ScanCallback;
import android.bluetooth.le.ScanResult;
import android.content.Context;
import android.os.Handler;
import android.os.Looper;
import android.util.Log;

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
        BluetoothManager bluetoothManager = (BluetoothManager) context.getSystemService(Context.BLUETOOTH_SERVICE);
        bluetoothAdapter = bluetoothManager.getAdapter();
        bluetoothLeScanner = bluetoothAdapter.getBluetoothLeScanner();
        Set<BluetoothDevice> bondedDevices = bluetoothAdapter.getBondedDevices();

        // check for bonded devices
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


    // device connected will register the callbacks here!
    // handle data in via the callback, or move it into my own gattcallback class?
    // https://developer.android.com/reference/android/bluetooth/BluetoothGattCallback
    private BluetoothGattCallback gattCallback = new BluetoothGattCallback() {
        @Override
        public void onConnectionStateChange(BluetoothGatt gatt, int status, int newState) {
            boolean connected = BluetoothGatt.GATT_SUCCESS == status;
            log.info("BLE connected to device: " + String.valueOf(connected));

            // TODO: move to function

            // get services
            BluetoothGattService s_data = gatt.getService(UUID.fromString("2000"));
            BluetoothGattService s_settings = gatt.getService(UUID.fromString("1000"));

            // get characteristics
                // data
            BluetoothGattCharacteristic c_speed = s_data.getCharacteristic(UUID.fromString("2001"));
            BluetoothGattCharacteristic c_trip_duration = s_data.getCharacteristic(UUID.fromString("2002"));
            BluetoothGattCharacteristic c_odometer = s_data.getCharacteristic(UUID.fromString("2003"));
            BluetoothGattCharacteristic c_temperature = s_data.getCharacteristic(UUID.fromString("2004"));
            BluetoothGattCharacteristic c_clock_minutes = s_data.getCharacteristic(UUID.fromString("2005"));
            BluetoothGattCharacteristic c_clock_hours = s_data.getCharacteristic(UUID.fromString("2006"));

            // set notify
                // data
            gatt.setCharacteristicNotification(c_speed, true);
            gatt.setCharacteristicNotification(c_trip_duration, true);
            gatt.setCharacteristicNotification(c_odometer, true);
            gatt.setCharacteristicNotification(c_temperature, true);
            gatt.setCharacteristicNotification(c_clock_minutes, true);
            gatt.setCharacteristicNotification(c_clock_hours, true);


            // for debug
            // possibly to setup notify?
            gatt.discoverServices();
        }

        @Override
        public void onCharacteristicChanged(BluetoothGatt gatt, BluetoothGattCharacteristic characteristic, byte[] value) {
            log.info("BLE characteristic changed");

        }

        @Override
        public void onCharacteristicRead(BluetoothGatt gatt, BluetoothGattCharacteristic characteristic, byte[] value, int status) {
            log.info("BLE characteristic read");
        }

        @Override
        public void onServicesDiscovered(BluetoothGatt gatt, int status) {
            log.info("BLE service discovered");

            // this is for debug only
            // in reality we will specify uuid's and ask notify them ourselves
            if (status == BluetoothGatt.GATT_SUCCESS) {
                List<BluetoothGattService> services = gatt.getServices(); // get list of all services
                //gatt.getService(UUID uuid); // get service by uuid
                log.info("BLE services:");
                for (BluetoothGattService s: services) {
                    log.info(s.getUuid().toString());
                    List<BluetoothGattCharacteristic> characteristics = s.getCharacteristics();

                    log.info("characteristics:");
                    for (BluetoothGattCharacteristic c : characteristics) {
                        log.info(c.getUuid().toString());
                    }
                    log.info("");
                }
            }

        }

    };


    public void connectDevice() {
        mDevice.createBond();
        mDevice.connectGatt(context, true, gattCallback);
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
