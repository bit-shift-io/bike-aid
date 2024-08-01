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
    protected static final UUID CHARACTERISTIC_UPDATE_NOTIFICATION_DESCRIPTOR_UUID = UUID.fromString("00002902-0000-1000-8000-00805f9b34fb");

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


    // device connected will register the callbacks here!
    // handle data in via the callback, or move it into my own gattcallback class?
    // https://developer.android.com/reference/android/bluetooth/BluetoothGattCallback
    private BluetoothGattCallback gattCallback = new BluetoothGattCallback() {
        @Override
        public void onConnectionStateChange(BluetoothGatt gatt, int status, int newState) {
            boolean connected = BluetoothGatt.GATT_SUCCESS == status;
            log.info("BLE connected to device: " + String.valueOf(connected));

            // now we need to scan the services
            gatt.discoverServices();
        }

        @Override
        public void onCharacteristicChanged(BluetoothGatt gatt, BluetoothGattCharacteristic characteristic, byte[] value) {
            // we were notified, now read the value
            gatt.readCharacteristic(characteristic);
        }

        @Override
        public void onCharacteristicRead(BluetoothGatt gatt, BluetoothGattCharacteristic characteristic, byte[] value, int status) {
            UUID id = characteristic.getUuid();
            String val = characteristic.getStringValue(0);
            log.info("BLE: id " + id);
            log.info("BLE: read " + val);
        }

        @Override
        public void onServicesDiscovered(BluetoothGatt gatt, int status) {
            log.info("BLE service discovered");

            // find all characteristics
            // then try to set notification on each one
            if (status == BluetoothGatt.GATT_SUCCESS) {
                List<BluetoothGattService> services = gatt.getServices(); // get list of all services
                log.info("BLE service scan begin...");

                for (BluetoothGattService s: services) {
                    List<BluetoothGattCharacteristic> characteristics = s.getCharacteristics();

                    for (BluetoothGattCharacteristic c : characteristics) {
                        setCharacteristicNotification(
                                gatt,
                                s.getUuid(),  // service
                                c.getUuid(),  // characteristic
                                true);
                        log.info(c.getUuid().toString());
                    }
                    log.info("");
                }

                log.info("BLE: service scan complete");
            }

        }

    };


    // setup notifications
    public boolean setCharacteristicNotification(BluetoothGatt gatt, UUID serviceUuid, UUID characteristicUuid, boolean enable) {
        BluetoothGattCharacteristic characteristic = gatt.getService(serviceUuid).getCharacteristic(characteristicUuid);
        gatt.setCharacteristicNotification(characteristic, enable);
        BluetoothGattDescriptor descriptor = characteristic.getDescriptor(CHARACTERISTIC_UPDATE_NOTIFICATION_DESCRIPTOR_UUID);
        if (descriptor == null)
            return false;
        byte[] val = enable ? BluetoothGattDescriptor.ENABLE_NOTIFICATION_VALUE : BluetoothGattDescriptor.DISABLE_NOTIFICATION_VALUE;
        descriptor.setValue(val);
        return gatt.writeDescriptor(descriptor); //descriptor write operation successfully started?
    }


    // Converts 16bit UUIDs to 128-bit format
    // the 16bit uuid is short for 0000xxxx-0000-1000-8000-00805F9B34FB
    public static UUID uuidFrom16(String uuid16) {
        String baseUUIDSuffix = "0000-1000-8000-00805F9B34FB";
        String uuid = "0000" + uuid16 + baseUUIDSuffix;
        return UUID.fromString(uuid);
    }

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
