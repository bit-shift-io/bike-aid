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

import java.nio.charset.StandardCharsets;
import java.util.ArrayList;
import java.util.List;
import java.util.Set;
import java.util.UUID;

public class BLE {
    // ==== notes ====
    /*
    This class handles the ble adaptor and connection to the device
    There are callbacks at the bottom which handle the gatt server and notifications

    Basically Ive simplified the android api to:
    init()
    connect() - automatic
    disconnect() - automatic
    onRead()
    write()

     */

    // ==== variables ====
    private static final BLE mInstance = new BLE();
    private static final Signals signals = Signals.getInstance();
    private static final Logger log = Logger.getInstance();
    private BluetoothLeScanner mScanner;
    private BluetoothAdapter mAdapter;
    private boolean mScanning;
    private final Handler handler = new Handler();
    private static final long SCAN_PERIOD = 60000; // 60 seconds
    private final String DEVICE_NAME = "BScooter";
    BluetoothDevice mDevice;
    BluetoothGatt mGatt;
    boolean mDeviceFound = false;
    private Context mContext;
    final ArrayList<BluetoothGattCharacteristic> mProcessedCharacteristics = new ArrayList<>();


    // ==== listener interface ====
    private OnEventListener mOnEventListener;
    public void setOnEventListener(OnEventListener listener) {
        mOnEventListener = listener;
    }
    public interface OnEventListener {
        void onRead(BluetoothGatt gatt, BluetoothGattCharacteristic characteristic, byte[] value, int status);
    }



    // ==== functions ====

    private BLE() {}


    public static BLE getInstance() {
        return mInstance;
    }


    public void init(Context c) {
        mContext = c;
        BluetoothManager bluetoothManager = (BluetoothManager) mContext.getSystemService(Context.BLUETOOTH_SERVICE);
        mAdapter = bluetoothManager.getAdapter();
        connect();
    }


    public void connect() {

        // check if bluetooth is on
        // apparently doesnt work on newer builds??
        if (!mAdapter.isEnabled()) {
            log.info("enable adapter");
            mAdapter.enable();

            String intentString = BluetoothAdapter.ACTION_REQUEST_ENABLE;
        }


        // check for bonded devices
        Set<BluetoothDevice> bondedDevices = mAdapter.getBondedDevices();
        if (bondedDevices != null) {
            for (BluetoothDevice dev : bondedDevices) {
                if (isMyDevice(dev)) {
                    connectDevice();
                    return;
                }
            }
        }

        // scan
        mScanner = mAdapter.getBluetoothLeScanner();
        if (mScanner == null) {
            log.info("no bluetooth adapter available, please enable bluetooth");
            return;
        }

        // device not found, scan
        if (!mDeviceFound)
            startScan();
    }

    public void connectDevice() {
        mDevice.createBond();
        // device connected will register the callbacks here
        // and so i have the gatt class to handle it
        mGatt = mDevice.connectGatt(mContext, true, mGattCallback);
    }


    // find if this is the wanted device, by name
    public boolean isMyDevice(BluetoothDevice dev) {
        if (dev == null || dev.getName() == null)
            return false;

        if (!dev.getName().equals(DEVICE_NAME))
            return false;

        mDevice = dev;
        mDeviceFound = true;
        log.info("device found");
        return true;
    }



    public void startScan() {
        if (!mScanning) {
            // Stops scanning after a predefined scan period.
            handler.postDelayed(new Runnable() {
                @Override
                public void run() {
                    log.info("stop scan");
                    mScanning = false;
                    mScanner.stopScan(mScanCallback);
                }
            }, SCAN_PERIOD);

            log.info("start scan");
            mScanning = true;
            mScanner.startScan(mScanCallback);
        } else {
            stopScan();
        }

    }

    public void stopScan() {
        mScanning = false;
        mScanner.stopScan(mScanCallback);
    }



    // ==== read & write functions ====

    public void onRead(BluetoothGatt gatt, BluetoothGattCharacteristic characteristic, byte[] value, int status) {
        mOnEventListener.onRead(gatt, characteristic, value, status);
    }


    public void write(UUID service, UUID characteristic, byte[] value) {
        if (mGatt == null)
            return;
        BluetoothGattCharacteristic c = mGatt.getService(service).getCharacteristic(characteristic);
        mGatt.writeCharacteristic(c, value , BluetoothGattCharacteristic.WRITE_TYPE_DEFAULT);
    }



    // ==== callbacks ====


    // scan call back
    private final ScanCallback mScanCallback = new ScanCallback() {
        @Override
        public void onScanResult(int callbackType, ScanResult result) {
            //super.onScanResult(callbackType, result);
            BluetoothDevice dev = result.getDevice();
            if (isMyDevice(dev)) {
                connectDevice();
                stopScan();
            }
        }
    };


    // gatt callback
    private final BluetoothGattCallback mGattCallback = new BluetoothGattCallback() {

        public void onCharacteristicWrite (BluetoothGatt gatt, BluetoothGattCharacteristic characteristic, int status) {
            if (status == BluetoothGatt.GATT_FAILURE)
                log.info("write fail!");
            else
                log.info("write success");
        }

        @Override
        public void onConnectionStateChange(BluetoothGatt gatt, int status, int newState) {
            // TODO: add reconnect code here
            if (status == BluetoothGatt.GATT_SUCCESS) {
                log.info("connected: " + mDevice.getName());
                gatt.discoverServices();
            }
        }

        @Override
        public void onCharacteristicChanged(BluetoothGatt gatt, BluetoothGattCharacteristic characteristic, byte[] value) {
            gatt.readCharacteristic(characteristic);
        }

        @Override
        public void onCharacteristicRead(BluetoothGatt gatt, BluetoothGattCharacteristic characteristic, byte[] value, int status) {
            onRead(gatt, characteristic, value, status);
        }

        @Override
        public void onDescriptorWrite (BluetoothGatt gatt, BluetoothGattDescriptor descriptor, int status) {
            String s = Functions.string16FromUUID(descriptor.getCharacteristic().getUuid());
            log.info("notify: " + s);
            processNextCharacteristic(gatt);
        }

        @Override
        public void onServicesDiscovered(BluetoothGatt gatt, int status) {
            processNextCharacteristic(gatt);
        }

        public boolean writeCharacteristicNotification(BluetoothGatt gatt, BluetoothGattCharacteristic characteristic, boolean enable) {
            gatt.setCharacteristicNotification(characteristic, enable);

            // write to descriptor for notification
            UUID CHARACTERISTIC_UPDATE_NOTIFICATION_DESCRIPTOR_UUID = UUID.fromString("00002902-0000-1000-8000-00805f9b34fb");
            BluetoothGattDescriptor descriptor = characteristic.getDescriptor(CHARACTERISTIC_UPDATE_NOTIFICATION_DESCRIPTOR_UUID);
            if (descriptor == null)
                return false;

            byte[] val = enable ? BluetoothGattDescriptor.ENABLE_NOTIFICATION_VALUE : BluetoothGattDescriptor.DISABLE_NOTIFICATION_VALUE;
            int result = gatt.writeDescriptor(descriptor, val);
            return true;
        }

        // this function will process the gatt characteristics descriptor
        // there is a delay with BLE so we need to wait till we receive the ok
        // then process the next in the list
        public void processNextCharacteristic(BluetoothGatt gatt) {
            // get list of all services
            List<BluetoothGattService> services = gatt.getServices();

            // loop services
            for (BluetoothGattService s : services) {
                List<BluetoothGattCharacteristic> characteristics = s.getCharacteristics();

                // loop characteristics
                for (BluetoothGattCharacteristic c : characteristics) {

                    // have we already been configured?
                    if (mProcessedCharacteristics.contains(c))
                        continue;

                    // set notification
                    boolean hasNotify = writeCharacteristicNotification(gatt, c, true);

                    // add to completed list
                    mProcessedCharacteristics.add(c);

                    // has notify, so wait for response
                    // if not try another characteristic
                    if (hasNotify)
                        return;
                }
            }

            // TODO: want to read initial values also!
            // so clear the mProcessedCharacteristics array, and start a second time to read values
            log.info("finish process characteristics");
        }

    };

}
